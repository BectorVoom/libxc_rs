//! GGA_X_ITYH kernel -- incremental derivative structure.

//! unpol: preamble=71 lines
//!   exc: shared=0, delta=71, outputs=1
//!   vxc: shared=71, delta=70, outputs=3
//!   fxc: shared=141, delta=152, outputs=6
//!   kxc: shared=293, delta=223, outputs=10
//!   lxc: shared=516, delta=163, outputs=15
//! pol: preamble=136 lines
//!   exc: shared=0, delta=136, outputs=1
//!   vxc: shared=136, delta=197, outputs=6
//!   fxc: shared=333, delta=556, outputs=21
//!   kxc: shared=889, delta=1046, outputs=56
//!   lxc: shared=1935, delta=1383, outputs=126

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
