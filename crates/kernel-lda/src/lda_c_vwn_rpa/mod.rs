//! LDA_C_VWN_RPA kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=38 lines
//!   exc: shared=0, delta=38, outputs=1
//!   vxc: shared=38, delta=49, outputs=2
//!   fxc: shared=87, delta=80, outputs=3
//!   kxc: shared=167, delta=125, outputs=4
//!   lxc: shared=292, delta=72, outputs=5
//! pol: preamble=52 lines
//!   exc: shared=0, delta=52, outputs=1
//!   vxc: shared=52, delta=69, outputs=3
//!   fxc: shared=121, delta=128, outputs=6
//!   kxc: shared=249, delta=194, outputs=10
//!   lxc: shared=443, delta=139, outputs=15

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
