//! GGA_X_LB kernel -- incremental derivative structure.

//! unpol: preamble=29 lines
//!   vxc: shared=0, delta=29, outputs=1
//!   fxc: shared=29, delta=34, outputs=3
//!   kxc: shared=63, delta=52, outputs=6
//!   lxc: shared=115, delta=25, outputs=10
//! pol: preamble=45 lines
//!   vxc: shared=0, delta=45, outputs=2
//!   fxc: shared=45, delta=58, outputs=11
//!   kxc: shared=103, delta=104, outputs=36
//!   lxc: shared=207, delta=105, outputs=91

pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
