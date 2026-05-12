//! LDA_XC_KSDT kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=185 lines
//!   exc: shared=0, delta=185, outputs=1
//!   vxc: shared=185, delta=223, outputs=2
//!   fxc: shared=408, delta=406, outputs=3
//!   kxc: shared=814, delta=783, outputs=4
//!   lxc: shared=1597, delta=366, outputs=5
//! pol: preamble=242 lines
//!   exc: shared=0, delta=242, outputs=1
//!   vxc: shared=242, delta=505, outputs=3
//!   fxc: shared=747, delta=1479, outputs=6
//!   kxc: shared=2226, delta=4214, outputs=10
//!   lxc: shared=6440, delta=7439, outputs=15

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol_part0_zk_vrho_v2rho2_v3rho3_0;
pub mod kxc_pol_part1_v3rho3_1;
pub mod kxc_pol_part2_v3rho3_2;
pub mod kxc_pol_part3_v3rho3_3;
pub mod lxc_pol_part0_zk_vrho_v2rho2_v3rho3_0;
pub mod lxc_pol_part1_v3rho3_1;
pub mod lxc_pol_part2_v3rho3_2;
pub mod lxc_pol_part3_v3rho3_3;
pub mod lxc_pol_part4_v4rho4_0;
pub mod lxc_pol_part5_v4rho4_1;
pub mod lxc_pol_part6_v4rho4_2;
pub mod lxc_pol_part7_v4rho4_3;
pub mod lxc_pol_part8_v4rho4_4;
