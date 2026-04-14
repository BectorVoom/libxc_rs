//! GGA_K_APBEINT kernel -- incremental derivative structure.

//! unpol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=42, outputs=3
//!   fxc: shared=83, delta=52, outputs=6
//!   kxc: shared=135, delta=68, outputs=10
//!   lxc: shared=203, delta=28, outputs=15
//! pol: preamble=66 lines
//!   exc: shared=0, delta=66, outputs=1
//!   vxc: shared=66, delta=86, outputs=6
//!   fxc: shared=152, delta=163, outputs=21
//!   kxc: shared=315, delta=281, outputs=56
//!   lxc: shared=596, delta=321, outputs=126

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
