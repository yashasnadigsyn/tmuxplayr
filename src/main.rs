use std::env;
use serde_json;
use reqwest::Client;
use std::io;
use std::process::Command;
use std::fs::OpenOptions;
use std::fs;
use std::io::Write;

#[tokio::main]
async fn main() {
    if let Err(e) = Command::new("mpv").status() {
        println!("mpv is needed to play music on termux, but mpv is not installed on your device.");
        println!("Do you want to install mpv?[Y/n]");
        let mut mpv_dl = String::new();
        io::stdin()
        .read_line(&mut mpv_dl)
        .expect("Error to read input");
        if mpv_dl.trim().to_lowercase() == "y" {
            Command::new("pkg").arg("install").arg("mpv").status().expect("error installing mpv!");
        } else {
            println!("Abort!")
        }
    } else {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1].to_lowercase() == "play" {
            let mut music_name = String::new();
            for i in 2..args.len() {
                music_name += &args[i];
                music_name += " ";
            }
            let client = Client::new();
            let url = ["http://127.0.0.1:5100/result/?query=", &music_name, "&lyrics=true"].join("");
            let res = client
                .get(url)
                .send()
                .await
                .expect("failed to get response")
                .json::<serde_json::Value>()
                .await.
                expect("error");
            let json_value = serde_json::json!(res);
            let mut media_url_vec = Vec::new();
            for i in 0..10 {
                if (&json_value[i]["song"]).to_string() == "null" {} else {
                    println!("{} --> {}", i, &json_value[i]["song"]);
                    media_url_vec.push(&json_value[i]["media_url"]);
                }
            }  
            println!("Please enter the index of the song you want to listen: ");
            let mut user_choice = String::new();
            io::stdin()
            .read_line(&mut user_choice)
            .expect("Error to read input");
            let user_choice: usize = user_choice.trim().parse().expect("Invalid index");
            if user_choice > media_url_vec.len() {
                println!("Index not in range!");
            } else {
                let mut danka = &mut media_url_vec[user_choice].to_string();
                danka.pop();
                danka.remove(0);
                println!("Use 'p' to pause, 'q' to quit and arrow keys to forward the song.");
                let status = Command::new("mpv").arg(danka).status().expect("error status");
            }
        } else if args[1].to_lowercase() == "add" {
            Command::new("mkdir").arg("tmuxplayr_playlist").status().expect("Error at creating playlist");
            let mut music_name = String::new();
            for i in 2..args.len() {
                music_name += &args[i];
                music_name += " ";
            }
            let client = Client::new();
            let url = ["http://127.0.0.1:5100/result/?query=", &music_name, "&lyrics=true"].join("");
            let res = client
                .get(url)
                .send()
                .await
                .expect("failed to get response")
                .json::<serde_json::Value>()
                .await
                .expect("error");
            let json_value = serde_json::json!(res);
            let mut media_url_vec = Vec::new();
            for i in 0..10 {
                if (&json_value[i]["song"]).to_string() == "null" {} else {
                    println!("{} --> {}", i, &json_value[i]["song"]);
                    media_url_vec.push(&json_value[i]["media_url"]);
                }
            }  
            println!("Please enter the index of the song you want to listen: ");
            let mut user_choice = String::new();
            io::stdin()
            .read_line(&mut user_choice)
            .expect("Error to read input");
            let user_choice: usize = user_choice.trim().parse().expect("Invalid index");
            if user_choice > media_url_vec.len() {
                println!("Index not in range!");
            } else {
                let mut danka = &mut media_url_vec[user_choice].to_string();
                danka.pop();
                danka.remove(0);
                println!("For which playlist you want to add?\nNote: if you gave a new name, a new playlist will be created else add to playlist listed.");
                let paths = fs::read_dir("tmuxplayr_playlist/").unwrap();
                for path in paths {
                    println!("{}", path.unwrap().path().display());
                }
                println!("So, which playlist you want to add?");
                let mut user_choice = String::new();
                io::stdin().read_line(&mut user_choice).expect("Error to read input");
                let mut user_choice = ["tmuxplayr_playlist/".to_string(), user_choice.trim().to_string(), ".m3u".to_string()].join("");
                let mut add_playlist = OpenOptions::new().append(true).create(true).open(user_choice).expect("error to add playlist");
                add_playlist.write_all(danka.as_bytes()).expect("failed to write file");              
                add_playlist.write_all("\n".as_bytes()).expect("failed to write file");          
            }
        } else if args[1].to_lowercase() == "help" {
            println!("help");
        } else if args[1].to_lowercase() == "playlist" {
            let paths = fs::read_dir("tmuxplayr_playlist/").unwrap();
            for path in paths {
                    println!("{}", path.unwrap().path().display());
                }
            println!("Which playlist?");
            let mut user_choice = String::new();
            io::stdin().read_line(&mut user_choice).expect("Error to read input");
            let mut user_choice = ["tmuxplayr_playlist/".to_string(), user_choice.trim().to_string(), ".m3u".to_string()].join("");
            Command::new("mpv").arg(user_choice).status().expect("Error to play the playlist!");
        } else {
            println!("Help");
        }
    } else {
        println!("No Arguements provided!");
    }
}
}