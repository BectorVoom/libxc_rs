//! LDA_C_CHACHIYO_MOD kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=28 lines
//!   exc: shared=0, delta=28, outputs=1
//!   vxc: shared=28, delta=9, outputs=2
//!   fxc: shared=37, delta=16, outputs=3
//!   kxc: shared=53, delta=16, outputs=4
//!   lxc: shared=69, delta=15, outputs=5
//! pol: preamble=43 lines
//!   exc: shared=0, delta=43, outputs=1
//!   vxc: shared=43, delta=31, outputs=3
//!   fxc: shared=74, delta=66, outputs=6
//!   kxc: shared=140, delta=109, outputs=10
//!   lxc: shared=249, delta=93, outputs=15

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
