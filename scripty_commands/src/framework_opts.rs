use crate::cmds;
use crate::error::on_error;
use crate::handler::event_listener;
use poise::{FrameworkOptions, PrefixFrameworkOptions};
use serenity::builder::{CreateAllowedMentions, ParseValue};

pub fn get_framework_opts() -> FrameworkOptions<crate::Data, crate::Error> {
    FrameworkOptions {
        commands: vec![
            cmds::setup(),
            cmds::register_cmds(),
            cmds::help(),
            cmds::join(),
            cmds::data_storage(),
            cmds::donate(),
            cmds::ping(),
            poise::Command {
                subcommands: vec![cmds::user_language(), cmds::guild_language()],
                ..cmds::language()
            },
            poise::Command {
                subcommands: vec![cmds::block_user(), cmds::block_guild()],
                ..cmds::block()
            },
        ],
        listener: |ctx, event, framework, user_data| {
            Box::pin(event_listener(ctx, event, framework, user_data))
        },
        on_error: |error| Box::pin(on_error(error)),
        command_check: Some(crate::entity_block::check_block),
        post_command: crate::handler::post_command,
        allowed_mentions: Some({
            let mut f = CreateAllowedMentions::default();
            // Only support direct user pings by default
            f.empty_parse().parse(ParseValue::Users);
            f
        }),
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("~".to_string()),
            execute_self_messages: false,
            execute_untracked_edits: true,
            mention_as_prefix: true,
            ..Default::default()
        },
        owners: scripty_config::get_config()
            .owners
            .iter()
            .map(|id| serenity::model::id::UserId(*id))
            .collect(),

        ..Default::default()
    }
}
