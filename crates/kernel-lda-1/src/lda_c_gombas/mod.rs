//! LDA_C_GOMBAS kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=8 lines
//!   exc: shared=0, delta=8, outputs=1
//!   vxc: shared=8, delta=10, outputs=2
//!   fxc: shared=18, delta=16, outputs=3
//!   kxc: shared=34, delta=23, outputs=4
//!   lxc: shared=57, delta=5, outputs=5
//! pol: preamble=9 lines
//!   exc: shared=0, delta=9, outputs=1
//!   vxc: shared=9, delta=11, outputs=3
//!   fxc: shared=20, delta=18, outputs=6
//!   kxc: shared=38, delta=26, outputs=10
//!   lxc: shared=64, delta=9, outputs=15

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
