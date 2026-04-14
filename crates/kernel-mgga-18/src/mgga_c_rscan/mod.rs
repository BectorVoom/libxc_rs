//! MGGA_C_RSCAN kernel -- incremental derivative structure.

//! unpol: preamble=125 lines
//!   exc: shared=0, delta=125, outputs=1
//!   vxc: shared=125, delta=163, outputs=5
//!   fxc: shared=288, delta=437, outputs=15
//!   kxc: shared=725, delta=967, outputs=35
//!   lxc: shared=1692, delta=905, outputs=70
//! pol: preamble=188 lines
//!   exc: shared=0, delta=188, outputs=1
//!   vxc: shared=188, delta=335, outputs=10
//!   fxc: shared=523, delta=1339, outputs=55
//!   kxc: shared=1862, delta=5171, outputs=220
//!   lxc: shared=7033, delta=12276, outputs=715

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
