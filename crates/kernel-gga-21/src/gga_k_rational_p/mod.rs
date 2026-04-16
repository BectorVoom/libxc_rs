//! GGA_K_RATIONAL_P kernel -- incremental derivative structure.

//! unpol: preamble=30 lines
//!   exc: shared=0, delta=30, outputs=1
//!   vxc: shared=30, delta=12, outputs=3
//!   fxc: shared=42, delta=33, outputs=6
//!   kxc: shared=75, delta=46, outputs=10
//!   lxc: shared=121, delta=37, outputs=15
//! pol: preamble=59 lines
//!   exc: shared=0, delta=59, outputs=1
//!   vxc: shared=59, delta=56, outputs=6
//!   fxc: shared=115, delta=126, outputs=21
//!   kxc: shared=241, delta=262, outputs=56
//!   lxc: shared=503, delta=390, outputs=126

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
