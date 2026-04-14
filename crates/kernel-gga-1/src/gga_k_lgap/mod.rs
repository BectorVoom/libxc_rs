//! GGA_K_LGAP kernel -- incremental derivative structure.

//! unpol: preamble=42 lines
//!   exc: shared=0, delta=42, outputs=1
//!   vxc: shared=42, delta=20, outputs=3
//!   fxc: shared=62, delta=27, outputs=6
//!   kxc: shared=89, delta=39, outputs=10
//!   lxc: shared=128, delta=22, outputs=15
//! pol: preamble=72 lines
//!   exc: shared=0, delta=72, outputs=1
//!   vxc: shared=72, delta=61, outputs=6
//!   fxc: shared=133, delta=131, outputs=21
//!   kxc: shared=264, delta=236, outputs=56
//!   lxc: shared=500, delta=310, outputs=126

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
