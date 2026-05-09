//! LDA_C_RPA kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=13 lines
//!   exc: shared=0, delta=13, outputs=1
//!   vxc: shared=13, delta=5, outputs=2
//!   fxc: shared=18, delta=6, outputs=3
//!   kxc: shared=24, delta=6, outputs=4
//!   lxc: shared=30, delta=3, outputs=5
//! pol: preamble=14 lines
//!   exc: shared=0, delta=14, outputs=1
//!   vxc: shared=14, delta=6, outputs=3
//!   fxc: shared=20, delta=8, outputs=6
//!   kxc: shared=28, delta=9, outputs=10
//!   lxc: shared=37, delta=7, outputs=15

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
