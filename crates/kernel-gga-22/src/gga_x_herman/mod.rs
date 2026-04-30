//! GGA_X_HERMAN kernel -- incremental derivative structure.

//! unpol: preamble=24 lines
//!   exc: shared=0, delta=24, outputs=1
//!   vxc: shared=24, delta=8, outputs=3
//!   fxc: shared=32, delta=7, outputs=6
//!   kxc: shared=39, delta=7, outputs=10
//!   lxc: shared=46, delta=7, outputs=15
//! pol: preamble=48 lines
//!   exc: shared=0, delta=48, outputs=1
//!   vxc: shared=48, delta=42, outputs=6
//!   fxc: shared=90, delta=88, outputs=21
//!   kxc: shared=178, delta=159, outputs=56
//!   lxc: shared=337, delta=204, outputs=126

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
