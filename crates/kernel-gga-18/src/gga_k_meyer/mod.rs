//! GGA_K_MEYER kernel -- incremental derivative structure.

//! unpol: preamble=53 lines
//!   exc: shared=0, delta=53, outputs=1
//!   vxc: shared=53, delta=47, outputs=3
//!   fxc: shared=100, delta=93, outputs=6
//!   kxc: shared=193, delta=193, outputs=10
//!   lxc: shared=386, delta=143, outputs=15
//! pol: preamble=89 lines
//!   exc: shared=0, delta=89, outputs=1
//!   vxc: shared=89, delta=99, outputs=6
//!   fxc: shared=188, delta=229, outputs=21
//!   kxc: shared=417, delta=494, outputs=56
//!   lxc: shared=911, delta=483, outputs=126

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
