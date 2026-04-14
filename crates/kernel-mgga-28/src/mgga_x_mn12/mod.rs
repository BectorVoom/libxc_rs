//! MGGA_X_MN12 kernel -- incremental derivative structure.

//! unpol: preamble=153 lines
//!   exc: shared=0, delta=153, outputs=1
//!   vxc: shared=153, delta=211, outputs=5
//!   fxc: shared=364, delta=360, outputs=15
//!   kxc: shared=724, delta=508, outputs=35
//!   lxc: shared=1232, delta=347, outputs=70
//! pol: preamble=270 lines
//!   exc: shared=0, delta=270, outputs=1
//!   vxc: shared=270, delta=336, outputs=10
//!   fxc: shared=606, delta=754, outputs=55
//!   kxc: shared=1360, delta=1605, outputs=220
//!   lxc: shared=2965, delta=1968, outputs=715

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
