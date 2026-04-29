//! GGA_XC_B97 kernel -- incremental derivative structure.

//! unpol: preamble=151 lines
//!   exc: shared=0, delta=151, outputs=1
//!   vxc: shared=151, delta=142, outputs=3
//!   fxc: shared=293, delta=213, outputs=6
//!   kxc: shared=506, delta=249, outputs=10
//!   lxc: shared=755, delta=173, outputs=15
//! pol: preamble=278 lines
//!   exc: shared=0, delta=278, outputs=1
//!   vxc: shared=278, delta=398, outputs=6
//!   fxc: shared=676, delta=892, outputs=21
//!   kxc: shared=1568, delta=1826, outputs=56
//!   lxc: shared=3394, delta=2237, outputs=126

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
