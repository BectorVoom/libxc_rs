//! GGA_C_PBE_VWN kernel -- incremental derivative structure.

//! unpol: preamble=85 lines
//!   exc: shared=0, delta=85, outputs=1
//!   vxc: shared=85, delta=100, outputs=3
//!   fxc: shared=185, delta=182, outputs=6
//!   kxc: shared=367, delta=342, outputs=10
//!   lxc: shared=709, delta=203, outputs=15
//! pol: preamble=126 lines
//!   exc: shared=0, delta=126, outputs=1
//!   vxc: shared=126, delta=221, outputs=6
//!   fxc: shared=347, delta=558, outputs=21
//!   kxc: shared=905, delta=1458, outputs=56
//!   lxc: shared=2363, delta=2100, outputs=126

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
