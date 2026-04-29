//! GGA_C_PBE kernel -- incremental derivative structure.

//! unpol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=80, outputs=3
//!   fxc: shared=150, delta=147, outputs=6
//!   kxc: shared=297, delta=263, outputs=10
//!   lxc: shared=560, delta=162, outputs=15
//! pol: preamble=105 lines
//!   exc: shared=0, delta=105, outputs=1
//!   vxc: shared=105, delta=180, outputs=6
//!   fxc: shared=285, delta=492, outputs=21
//!   kxc: shared=777, delta=1313, outputs=56
//!   lxc: shared=2090, delta=2070, outputs=126

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
