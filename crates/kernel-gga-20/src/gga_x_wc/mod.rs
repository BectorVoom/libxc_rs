//! GGA_X_WC kernel -- incremental derivative structure.

//! unpol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=26, outputs=3
//!   fxc: shared=67, delta=52, outputs=6
//!   kxc: shared=119, delta=61, outputs=10
//!   lxc: shared=180, delta=31, outputs=15
//! pol: preamble=68 lines
//!   exc: shared=0, delta=68, outputs=1
//!   vxc: shared=68, delta=59, outputs=6
//!   fxc: shared=127, delta=155, outputs=21
//!   kxc: shared=282, delta=273, outputs=56
//!   lxc: shared=555, delta=331, outputs=126

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
