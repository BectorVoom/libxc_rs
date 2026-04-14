//! MGGA_X_TPSS kernel -- incremental derivative structure.

//! unpol: preamble=86 lines
//!   exc: shared=0, delta=86, outputs=1
//!   vxc: shared=86, delta=107, outputs=5
//!   fxc: shared=193, delta=230, outputs=15
//!   kxc: shared=423, delta=492, outputs=35
//!   lxc: shared=915, delta=465, outputs=70
//! pol: preamble=150 lines
//!   exc: shared=0, delta=150, outputs=1
//!   vxc: shared=150, delta=211, outputs=10
//!   fxc: shared=361, delta=513, outputs=55
//!   kxc: shared=874, delta=1228, outputs=220
//!   lxc: shared=2102, delta=1757, outputs=715

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
