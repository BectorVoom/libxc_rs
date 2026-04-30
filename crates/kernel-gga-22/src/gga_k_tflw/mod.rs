//! GGA_K_TFLW kernel -- incremental derivative structure.

//! unpol: preamble=28 lines
//!   exc: shared=0, delta=28, outputs=1
//!   vxc: shared=28, delta=8, outputs=3
//!   fxc: shared=36, delta=7, outputs=6
//!   kxc: shared=43, delta=7, outputs=10
//!   lxc: shared=50, delta=7, outputs=15
//! pol: preamble=53 lines
//!   exc: shared=0, delta=53, outputs=1
//!   vxc: shared=53, delta=41, outputs=6
//!   fxc: shared=94, delta=86, outputs=21
//!   kxc: shared=180, delta=159, outputs=56
//!   lxc: shared=339, delta=225, outputs=126

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
