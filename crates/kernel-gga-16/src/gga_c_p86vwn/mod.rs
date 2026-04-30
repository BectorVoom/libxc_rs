//! GGA_C_P86VWN kernel -- incremental derivative structure.

//! unpol: preamble=76 lines
//!   exc: shared=0, delta=76, outputs=1
//!   vxc: shared=76, delta=82, outputs=3
//!   fxc: shared=158, delta=143, outputs=6
//!   kxc: shared=301, delta=240, outputs=10
//!   lxc: shared=541, delta=91, outputs=15
//! pol: preamble=121 lines
//!   exc: shared=0, delta=121, outputs=1
//!   vxc: shared=121, delta=174, outputs=6
//!   fxc: shared=295, delta=407, outputs=21
//!   kxc: shared=702, delta=899, outputs=56
//!   lxc: shared=1601, delta=1187, outputs=126

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
