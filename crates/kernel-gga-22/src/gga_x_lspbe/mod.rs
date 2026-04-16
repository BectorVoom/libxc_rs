//! GGA_X_LSPBE kernel -- incremental derivative structure.

//! unpol: preamble=32 lines
//!   exc: shared=0, delta=32, outputs=1
//!   vxc: shared=32, delta=22, outputs=3
//!   fxc: shared=54, delta=38, outputs=6
//!   kxc: shared=92, delta=39, outputs=10
//!   lxc: shared=131, delta=26, outputs=15
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=48, outputs=6
//!   fxc: shared=105, delta=112, outputs=21
//!   kxc: shared=217, delta=199, outputs=56
//!   lxc: shared=416, delta=258, outputs=126

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
