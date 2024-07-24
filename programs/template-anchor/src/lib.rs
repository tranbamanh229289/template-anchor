use anchor_lang::prelude::*;

declare_id!("FiifvSGXR8Q8BPCKTremuAkVXxuUefLGbhtw781kER1N");

#[program]
pub mod template_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
