//! GGA_X_PBETRANS kernel -- incremental derivative structure.

//! unpol: preamble=38 lines
//!   exc: shared=0, delta=38, outputs=1
//!   vxc: shared=38, delta=38, outputs=3
//!   fxc: shared=76, delta=81, outputs=6
//!   kxc: shared=157, delta=160, outputs=10
//!   lxc: shared=317, delta=172, outputs=15
//! pol: preamble=67 lines
//!   exc: shared=0, delta=67, outputs=1
//!   vxc: shared=67, delta=75, outputs=6
//!   fxc: shared=142, delta=173, outputs=21
//!   kxc: shared=315, delta=415, outputs=56
//!   lxc: shared=730, delta=542, outputs=126

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
