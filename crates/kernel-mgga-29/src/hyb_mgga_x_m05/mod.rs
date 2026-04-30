//! HYB_MGGA_X_M05 kernel -- incremental derivative structure.

//! unpol: preamble=87 lines
//!   exc: shared=0, delta=87, outputs=1
//!   vxc: shared=87, delta=84, outputs=5
//!   fxc: shared=171, delta=151, outputs=15
//!   kxc: shared=322, delta=219, outputs=35
//!   lxc: shared=541, delta=223, outputs=70
//! pol: preamble=154 lines
//!   exc: shared=0, delta=154, outputs=1
//!   vxc: shared=154, delta=172, outputs=10
//!   fxc: shared=326, delta=410, outputs=55
//!   kxc: shared=736, delta=839, outputs=220
//!   lxc: shared=1575, delta=1244, outputs=715

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
