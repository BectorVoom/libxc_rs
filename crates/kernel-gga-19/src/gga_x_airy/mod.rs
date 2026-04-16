//! GGA_X_AIRY kernel -- incremental derivative structure.

//! unpol: preamble=36 lines
//!   exc: shared=0, delta=36, outputs=1
//!   vxc: shared=36, delta=37, outputs=3
//!   fxc: shared=73, delta=76, outputs=6
//!   kxc: shared=149, delta=121, outputs=10
//!   lxc: shared=270, delta=122, outputs=15
//! pol: preamble=65 lines
//!   exc: shared=0, delta=65, outputs=1
//!   vxc: shared=65, delta=87, outputs=6
//!   fxc: shared=152, delta=189, outputs=21
//!   kxc: shared=341, delta=358, outputs=56
//!   lxc: shared=699, delta=435, outputs=126

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
