//! LDA_C_PW_ERF kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=106 lines
//!   exc: shared=0, delta=106, outputs=1
//!   vxc: shared=106, delta=89, outputs=2
//!   fxc: shared=195, delta=134, outputs=3
//!   kxc: shared=329, delta=193, outputs=4
//!   lxc: shared=522, delta=134, outputs=5
//! pol: preamble=187 lines
//!   exc: shared=0, delta=187, outputs=1
//!   vxc: shared=187, delta=326, outputs=3
//!   fxc: shared=513, delta=1026, outputs=6
//!   kxc: shared=1539, delta=2953, outputs=10
//!   lxc: shared=4492, delta=6718, outputs=15

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol_part0_zk_vrho_v2rho2;
pub mod lxc_pol_part1_v3rho3;
pub mod lxc_pol_part2_v4rho4_0;
pub mod lxc_pol_part3_v4rho4_1;
pub mod lxc_pol_part4_v4rho4_2;
pub mod lxc_pol_part5_v4rho4_3;
pub mod lxc_pol_part6_v4rho4_4;
