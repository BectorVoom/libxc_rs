//! GGA_X_RPBE kernel -- incremental derivative structure.

//! unpol: preamble=29 lines
//!   exc: shared=0, delta=29, outputs=1
//!   vxc: shared=29, delta=10, outputs=3
//!   fxc: shared=39, delta=21, outputs=6
//!   kxc: shared=60, delta=27, outputs=10
//!   lxc: shared=87, delta=15, outputs=15
//! pol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=46, outputs=6
//!   fxc: shared=97, delta=123, outputs=21
//!   kxc: shared=220, delta=238, outputs=56
//!   lxc: shared=458, delta=285, outputs=126

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
