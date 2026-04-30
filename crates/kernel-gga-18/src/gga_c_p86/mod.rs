//! GGA_C_P86 kernel -- incremental derivative structure.

//! unpol: preamble=61 lines
//!   exc: shared=0, delta=61, outputs=1
//!   vxc: shared=61, delta=49, outputs=3
//!   fxc: shared=110, delta=73, outputs=6
//!   kxc: shared=183, delta=116, outputs=10
//!   lxc: shared=299, delta=39, outputs=15
//! pol: preamble=85 lines
//!   exc: shared=0, delta=85, outputs=1
//!   vxc: shared=85, delta=88, outputs=6
//!   fxc: shared=173, delta=219, outputs=21
//!   kxc: shared=392, delta=517, outputs=56
//!   lxc: shared=909, delta=830, outputs=126

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
