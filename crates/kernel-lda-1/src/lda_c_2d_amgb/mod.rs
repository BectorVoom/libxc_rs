//! LDA_C_2D_AMGB kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=21 lines
//!   exc: shared=0, delta=21, outputs=1
//!   vxc: shared=21, delta=18, outputs=2
//!   fxc: shared=39, delta=29, outputs=3
//!   kxc: shared=68, delta=39, outputs=4
//!   lxc: shared=107, delta=12, outputs=5
//! pol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=76, outputs=3
//!   fxc: shared=131, delta=157, outputs=6
//!   kxc: shared=288, delta=258, outputs=10
//!   lxc: shared=546, delta=248, outputs=15

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
