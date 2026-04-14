//! MGGA_X_BR89_EXPLICIT kernel -- incremental derivative structure.

//! unpol: preamble=72 lines
//!   exc: shared=0, delta=72, outputs=1
//!   vxc: shared=72, delta=163, outputs=5
//!   fxc: shared=235, delta=658, outputs=15
//!   kxc: shared=893, delta=2461, outputs=35
//!   lxc: shared=3354, delta=5357, outputs=70
//! pol: preamble=141 lines
//!   exc: shared=0, delta=141, outputs=1
//!   vxc: shared=141, delta=348, outputs=10
//!   fxc: shared=489, delta=1430, outputs=55
//!   kxc: shared=1919, delta=5489, outputs=220
//!   lxc: shared=7408, delta=13444, outputs=715

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
