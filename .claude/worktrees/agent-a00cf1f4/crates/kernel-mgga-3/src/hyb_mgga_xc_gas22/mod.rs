//! HYB_MGGA_XC_GAS22 kernel -- incremental derivative structure.

//! unpol: preamble=186 lines
//!   exc: shared=0, delta=186, outputs=1
//!   vxc: shared=186, delta=216, outputs=5
//!   fxc: shared=402, delta=406, outputs=15
//!   kxc: shared=808, delta=662, outputs=35
//!   lxc: shared=1470, delta=798, outputs=70
//! pol: preamble=330 lines
//!   exc: shared=0, delta=330, outputs=1
//!   vxc: shared=330, delta=565, outputs=10
//!   fxc: shared=895, delta=1565, outputs=55
//!   kxc: shared=2460, delta=4258, outputs=220
//!   lxc: shared=6718, delta=8657, outputs=715

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
