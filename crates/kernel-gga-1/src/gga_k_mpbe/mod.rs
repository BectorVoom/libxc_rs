//! GGA_K_MPBE kernel -- incremental derivative structure.

//! unpol: preamble=53 lines
//!   exc: shared=0, delta=53, outputs=1
//!   vxc: shared=53, delta=38, outputs=3
//!   fxc: shared=91, delta=42, outputs=6
//!   kxc: shared=133, delta=58, outputs=10
//!   lxc: shared=191, delta=37, outputs=15
//! pol: preamble=89 lines
//!   exc: shared=0, delta=89, outputs=1
//!   vxc: shared=89, delta=85, outputs=6
//!   fxc: shared=174, delta=135, outputs=21
//!   kxc: shared=309, delta=245, outputs=56
//!   lxc: shared=554, delta=271, outputs=126

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
