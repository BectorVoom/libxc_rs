//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1118;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1119;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1120;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1121;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1122;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1123;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta169(t1398: f64, t543: f64, t550: f64, t3992: f64, t2661: f64, t1384: f64, t544: f64, t235: f64, t239: f64, t820: f64, t3923: f64, t1390: f64, t828: f64, t531: f64, t549: f64, t240: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3994, t3995, t3996, t3999) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1118(t1398, t543, t550, t3992, t2661, t1384, t544);
        let t4000 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1119(t235, t3999);
        let (t4002, t4003) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1120(t239, t4000, t820, t543);
        let t4004 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1121(t3923, t4003);
        let (t4006, t4010) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1122(t1390, t4004, t828, t531, t549);
        let t4011 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1123(t240, t4010);
        let t4012 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1124(t4011, t72);
    (t3994, t3995, t3996, t3999, t4000, t4002, t4003, t4004, t4006, t4010, t4011, t4012)
}
