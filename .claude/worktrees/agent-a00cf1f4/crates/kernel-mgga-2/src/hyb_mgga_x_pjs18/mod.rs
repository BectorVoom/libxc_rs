//! HYB_MGGA_X_PJS18 kernel -- incremental derivative structure.

//! unpol: preamble=185 lines
//!   exc: shared=0, delta=185, outputs=1
//!   vxc: shared=185, delta=242, outputs=5
//!   fxc: shared=427, delta=476, outputs=15
//!   kxc: shared=903, delta=701, outputs=35
//!   lxc: shared=1604, delta=648, outputs=70
//! pol: preamble=359 lines
//!   exc: shared=0, delta=359, outputs=1
//!   vxc: shared=359, delta=686, outputs=10
//!   fxc: shared=1045, delta=1906, outputs=55
//!   kxc: shared=2951, delta=3330, outputs=220
//!   lxc: shared=6281, delta=5554, outputs=715

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
