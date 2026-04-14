//! MGGA_X_MS kernel -- incremental derivative structure.

//! unpol: preamble=52 lines
//!   exc: shared=0, delta=52, outputs=1
//!   vxc: shared=52, delta=56, outputs=5
//!   fxc: shared=108, delta=155, outputs=15
//!   kxc: shared=263, delta=441, outputs=35
//!   lxc: shared=704, delta=365, outputs=70
//! pol: preamble=87 lines
//!   exc: shared=0, delta=87, outputs=1
//!   vxc: shared=87, delta=130, outputs=10
//!   fxc: shared=217, delta=395, outputs=55
//!   kxc: shared=612, delta=1102, outputs=220
//!   lxc: shared=1714, delta=1263, outputs=715

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
