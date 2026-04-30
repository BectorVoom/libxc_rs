//! GGA_X_CHACHIYO kernel -- incremental derivative structure.

//! unpol: preamble=32 lines
//!   exc: shared=0, delta=32, outputs=1
//!   vxc: shared=32, delta=27, outputs=3
//!   fxc: shared=59, delta=64, outputs=6
//!   kxc: shared=123, delta=148, outputs=10
//!   lxc: shared=271, delta=134, outputs=15
//! pol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=90, outputs=6
//!   fxc: shared=160, delta=238, outputs=21
//!   kxc: shared=398, delta=624, outputs=56
//!   lxc: shared=1022, delta=1263, outputs=126

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
