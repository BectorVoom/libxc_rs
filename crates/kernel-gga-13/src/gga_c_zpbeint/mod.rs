//! GGA_C_ZPBEINT kernel -- incremental derivative structure.

//! unpol: preamble=80 lines
//!   exc: shared=0, delta=80, outputs=1
//!   vxc: shared=80, delta=102, outputs=3
//!   fxc: shared=182, delta=215, outputs=6
//!   kxc: shared=397, delta=404, outputs=10
//!   lxc: shared=801, delta=205, outputs=15
//! pol: preamble=117 lines
//!   exc: shared=0, delta=117, outputs=1
//!   vxc: shared=117, delta=214, outputs=6
//!   fxc: shared=331, delta=695, outputs=21
//!   kxc: shared=1026, delta=2320, outputs=56
//!   lxc: shared=3346, delta=3512, outputs=126

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
