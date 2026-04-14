//! MGGA_X_SA_TPSS kernel -- incremental derivative structure.

//! unpol: preamble=88 lines
//!   exc: shared=0, delta=88, outputs=1
//!   vxc: shared=88, delta=144, outputs=5
//!   fxc: shared=232, delta=403, outputs=15
//!   kxc: shared=635, delta=1194, outputs=35
//!   lxc: shared=1829, delta=1340, outputs=70
//! pol: preamble=163 lines
//!   exc: shared=0, delta=163, outputs=1
//!   vxc: shared=163, delta=277, outputs=10
//!   fxc: shared=440, delta=837, outputs=55
//!   kxc: shared=1277, delta=2468, outputs=220
//!   lxc: shared=3745, delta=3380, outputs=715

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
