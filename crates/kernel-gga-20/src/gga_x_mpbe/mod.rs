//! GGA_X_MPBE kernel -- incremental derivative structure.

//! unpol: preamble=50 lines
//!   exc: shared=0, delta=50, outputs=1
//!   vxc: shared=50, delta=38, outputs=3
//!   fxc: shared=88, delta=42, outputs=6
//!   kxc: shared=130, delta=58, outputs=10
//!   lxc: shared=188, delta=37, outputs=15
//! pol: preamble=84 lines
//!   exc: shared=0, delta=84, outputs=1
//!   vxc: shared=84, delta=86, outputs=6
//!   fxc: shared=170, delta=137, outputs=21
//!   kxc: shared=307, delta=245, outputs=56
//!   lxc: shared=552, delta=271, outputs=126

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
