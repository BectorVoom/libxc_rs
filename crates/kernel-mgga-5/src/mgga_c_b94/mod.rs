//! MGGA_C_B94 kernel -- incremental derivative structure.

//! unpol: preamble=92 lines
//!   exc: shared=0, delta=92, outputs=1
//!   vxc: shared=92, delta=228, outputs=5
//!   fxc: shared=320, delta=1333, outputs=15
//!   kxc: shared=1653, delta=4457, outputs=35
//! pol: preamble=175 lines
//!   exc: shared=0, delta=175, outputs=1
//!   vxc: shared=175, delta=505, outputs=10
//!   fxc: shared=680, delta=3552, outputs=55
//!   kxc: shared=4232, delta=12172, outputs=220

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
