//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1781;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1782;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1783;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1784;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta375(t3373: f64, t689: f64, t159: f64, t3617: f64, t12257: f64, t128: f64, t12269: f64, t3360: f64, t1120: f64, t12273: f64, t12287: f64, t12277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12303 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1780(t3373, t689);
        let (t12305, t12306, t12307) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1781(t159, t3617, t12257, t128);
        let (t12309, t12310) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1782(t12269, t3360, t128);
        let (t12313, t12314) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1783(t1120, t12273, t128);
        let (t12316, t12317) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1784(t1120, t12287, t128);
        let (t12319, t12320) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1785(t1120, t12277, t128);
    (t12303, t12305, t12306, t12307, t12309, t12310, t12313, t12314, t12316, t12317, t12319, t12320)
}
