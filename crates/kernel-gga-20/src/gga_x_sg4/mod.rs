//! GGA_X_SG4 kernel -- incremental derivative structure.

//! unpol: preamble=46 lines
//!   exc: shared=0, delta=46, outputs=1
//!   vxc: shared=46, delta=24, outputs=3
//!   fxc: shared=70, delta=46, outputs=6
//!   kxc: shared=116, delta=62, outputs=10
//!   lxc: shared=178, delta=38, outputs=15
//! pol: preamble=78 lines
//!   exc: shared=0, delta=78, outputs=1
//!   vxc: shared=78, delta=63, outputs=6
//!   fxc: shared=141, delta=145, outputs=21
//!   kxc: shared=286, delta=248, outputs=56
//!   lxc: shared=534, delta=271, outputs=126

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
