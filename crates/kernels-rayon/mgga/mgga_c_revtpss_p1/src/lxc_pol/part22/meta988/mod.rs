//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta988 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3353;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3354;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3355;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3356;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3357;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3358;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta988(t18905: f64, t689: f64, t18903: f64, t2258: f64, t11142: f64, t128: f64, t2251: f64, t41296: f64, t5819: f64, t41339: f64, t18908: f64, t2850: f64, t13312: f64, t4573: f64, t18927: f64, t11150: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t63342 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3353(t18905, t689);
        let (t63344, t63346) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3354(t18903, t2258, t11142, t128);
        let (t63349, t63351) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3355(t2251, t41296, t5819, t128, t41339);
        let (t63353, t63355) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3356(t18908, t2258, t128, t2850);
        let (t63357, t63359) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3357(t13312, t4573, t128, t2850);
        let t63361 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3358(t18927, t689);
        let (t63364, t63366) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3359(t11150, t5825, t2251, t128, t2850);
    (t63342, t63344, t63346, t63349, t63351, t63353, t63355, t63357, t63359, t63361, t63364, t63366)
}
