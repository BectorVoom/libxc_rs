//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta374 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1774;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1775;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1776;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1777;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1778;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta374(t12282: f64, t3417: f64, t141: f64, t3367: f64, t606: f64, t2258: f64, t1145: f64, t3360: f64, t128: f64, t268: f64, t404: f64, t7021: f64, t1123: f64, t2435: f64, t3364: f64, t689: f64, t3369: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12283, t12284, t12287) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1774(t12282, t3417, t141, t3367, t606, t2258);
        let (t12288, t12289, t12291, t12292) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1775(t1145, t12287, t141, t12282, t3360, t128);
        let t12295 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1776(t268, t404, t7021);
        let (t12296, t12297) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1777(t12295, t1123, t2435);
        let t12299 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1778(t3364, t689);
        let t12301 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1779(t3369, t689);
    (t12283, t12284, t12287, t12288, t12289, t12291, t12292, t12295, t12296, t12297, t12299, t12301)
}
