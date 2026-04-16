//! LDA_X_2D kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=8 lines
//!   exc: shared=0, delta=8, outputs=1
//!   vxc: shared=8, delta=1, outputs=2
//!   fxc: shared=9, delta=1, outputs=3
//!   kxc: shared=10, delta=1, outputs=4
//!   lxc: shared=11, delta=2, outputs=5
//! pol: preamble=24 lines
//!   exc: shared=0, delta=24, outputs=1
//!   vxc: shared=24, delta=18, outputs=3
//!   fxc: shared=42, delta=33, outputs=6
//!   kxc: shared=75, delta=49, outputs=10
//!   lxc: shared=124, delta=47, outputs=15

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
