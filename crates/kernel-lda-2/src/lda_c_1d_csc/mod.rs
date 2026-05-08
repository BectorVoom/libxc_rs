//! LDA_C_1D_CSC kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=23 lines
//!   exc: shared=0, delta=23, outputs=1
//!   vxc: shared=23, delta=14, outputs=2
//!   fxc: shared=37, delta=27, outputs=3
//!   kxc: shared=64, delta=39, outputs=4
//!   lxc: shared=103, delta=15, outputs=5
//! pol: preamble=48 lines
//!   exc: shared=0, delta=48, outputs=1
//!   vxc: shared=48, delta=34, outputs=3
//!   fxc: shared=82, delta=80, outputs=6
//!   kxc: shared=162, delta=136, outputs=10
//!   lxc: shared=298, delta=143, outputs=15

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
