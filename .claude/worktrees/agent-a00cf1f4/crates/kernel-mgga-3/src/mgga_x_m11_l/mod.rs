//! MGGA_X_M11_L kernel -- incremental derivative structure.

//! unpol: preamble=198 lines
//!   exc: shared=0, delta=198, outputs=1
//!   vxc: shared=198, delta=231, outputs=5
//!   fxc: shared=429, delta=357, outputs=15
//!   kxc: shared=786, delta=401, outputs=35
//!   lxc: shared=1187, delta=554, outputs=70
//! pol: preamble=349 lines
//!   exc: shared=0, delta=349, outputs=1
//!   vxc: shared=349, delta=383, outputs=10
//!   fxc: shared=732, delta=798, outputs=55
//!   kxc: shared=1530, delta=1428, outputs=220
//!   lxc: shared=2958, delta=1849, outputs=715

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
