//! GGA_X_PW91 kernel -- incremental derivative structure.

//! unpol: preamble=46 lines
//!   exc: shared=0, delta=46, outputs=1
//!   vxc: shared=46, delta=46, outputs=3
//!   fxc: shared=92, delta=64, outputs=6
//!   kxc: shared=156, delta=97, outputs=10
//!   lxc: shared=253, delta=66, outputs=15
//! pol: preamble=79 lines
//!   exc: shared=0, delta=79, outputs=1
//!   vxc: shared=79, delta=105, outputs=6
//!   fxc: shared=184, delta=179, outputs=21
//!   kxc: shared=363, delta=324, outputs=56
//!   lxc: shared=687, delta=333, outputs=126

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
