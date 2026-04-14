//! MGGA_X_GX kernel -- incremental derivative structure.

//! unpol: preamble=52 lines
//!   exc: shared=0, delta=52, outputs=1
//!   vxc: shared=52, delta=60, outputs=5
//!   fxc: shared=112, delta=158, outputs=15
//!   kxc: shared=270, delta=340, outputs=35
//!   lxc: shared=610, delta=301, outputs=70
//! pol: preamble=88 lines
//!   exc: shared=0, delta=88, outputs=1
//!   vxc: shared=88, delta=128, outputs=10
//!   fxc: shared=216, delta=354, outputs=55
//!   kxc: shared=570, delta=868, outputs=220
//!   lxc: shared=1438, delta=1107, outputs=715

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
