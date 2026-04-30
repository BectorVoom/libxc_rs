//! GGA_K_APBE kernel -- incremental derivative structure.

//! unpol: preamble=28 lines
//!   exc: shared=0, delta=28, outputs=1
//!   vxc: shared=28, delta=13, outputs=3
//!   fxc: shared=41, delta=21, outputs=6
//!   kxc: shared=62, delta=28, outputs=10
//!   lxc: shared=90, delta=14, outputs=15
//! pol: preamble=56 lines
//!   exc: shared=0, delta=56, outputs=1
//!   vxc: shared=56, delta=52, outputs=6
//!   fxc: shared=108, delta=114, outputs=21
//!   kxc: shared=222, delta=216, outputs=56
//!   lxc: shared=438, delta=292, outputs=126

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
