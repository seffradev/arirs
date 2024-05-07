use arirs::{client::Client, Event};
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

const APP_NAME: &str = "ari";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(LevelFilter::TRACE)
        .init();

    let (tx, mut rx) = tokio::sync::mpsc::channel(1024);

    let client = Arc::new(
        Client::new()
            .url("http://localhost:8088/ari")?
            .username("asterisk")
            .password("asterisk")
            .app_name(APP_NAME)
            .handler(tx)
            .build()?,
    );

    let client_clone = client.clone();
    tokio::spawn(async move {
        if let Err(e) = client_clone.run().await {
            eprintln!("Error: {}", e);
        }
    });

    while let Some(event) = rx.recv().await {
        match event {
            Event::StasisStart(e) => {
                client
                    .play_media(
                        &e.channel.id,
                        "sound:hello",
                        Some("en"),
                        None,
                        None,
                        None,
                    )
                    .await?;
            }
            _ => {}
        }
    }

    Ok(())
}
