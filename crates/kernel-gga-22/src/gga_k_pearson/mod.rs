//! GGA_K_PEARSON kernel -- incremental derivative structure.

//! unpol: preamble=39 lines
//!   exc: shared=0, delta=39, outputs=1
//!   vxc: shared=39, delta=18, outputs=3
//!   fxc: shared=57, delta=29, outputs=6
//!   kxc: shared=86, delta=38, outputs=10
//!   lxc: shared=124, delta=15, outputs=15
//! pol: preamble=65 lines
//!   exc: shared=0, delta=65, outputs=1
//!   vxc: shared=65, delta=58, outputs=6
//!   fxc: shared=123, delta=116, outputs=21
//!   kxc: shared=239, delta=206, outputs=56
//!   lxc: shared=445, delta=239, outputs=126

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
