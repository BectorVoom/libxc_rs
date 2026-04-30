//! GGA_X_PBEINT kernel -- incremental derivative structure.

//! unpol: preamble=38 lines
//!   exc: shared=0, delta=38, outputs=1
//!   vxc: shared=38, delta=42, outputs=3
//!   fxc: shared=80, delta=52, outputs=6
//!   kxc: shared=132, delta=67, outputs=10
//!   lxc: shared=199, delta=28, outputs=15
//! pol: preamble=61 lines
//!   exc: shared=0, delta=61, outputs=1
//!   vxc: shared=61, delta=87, outputs=6
//!   fxc: shared=148, delta=165, outputs=21
//!   kxc: shared=313, delta=281, outputs=56
//!   lxc: shared=594, delta=321, outputs=126

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
