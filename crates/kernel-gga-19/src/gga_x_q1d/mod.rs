//! GGA_X_Q1D kernel -- incremental derivative structure.

//! unpol: preamble=54 lines
//!   exc: shared=0, delta=54, outputs=1
//!   vxc: shared=54, delta=40, outputs=3
//!   fxc: shared=94, delta=58, outputs=6
//!   kxc: shared=152, delta=103, outputs=10
//!   lxc: shared=255, delta=60, outputs=15
//! pol: preamble=90 lines
//!   exc: shared=0, delta=90, outputs=1
//!   vxc: shared=90, delta=97, outputs=6
//!   fxc: shared=187, delta=170, outputs=21
//!   kxc: shared=357, delta=339, outputs=56
//!   lxc: shared=696, delta=330, outputs=126

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
