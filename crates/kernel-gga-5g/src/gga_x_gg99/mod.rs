//! GGA_X_GG99 kernel -- incremental derivative structure.

//! unpol: preamble=87 lines
//!   exc: shared=0, delta=87, outputs=1
//!   vxc: shared=87, delta=82, outputs=3
//!   fxc: shared=169, delta=220, outputs=6
//!   kxc: shared=389, delta=532, outputs=10
//!   lxc: shared=921, delta=508, outputs=15
//! pol: preamble=155 lines
//!   exc: shared=0, delta=155, outputs=1
//!   vxc: shared=155, delta=182, outputs=6
//!   fxc: shared=337, delta=524, outputs=21
//!   kxc: shared=861, delta=1344, outputs=56
//!   lxc: shared=2205, delta=1816, outputs=126

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
