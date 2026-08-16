//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3050;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3051;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3052;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3053;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3054;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3055;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3056;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta822(t10356: f64, t16719: f64, t1120: f64, t128: f64, t1469: f64, t43766: f64, t43860: f64, t2435: f64, t5048: f64, t43776: f64, t12305: f64, t5053: f64, t16739: f64, t689: f64, t16743: f64, t16734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56165, t56167) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3050(t10356, t16719, t1120, t128);
        let (t56172, t56174) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3051(t10356, t1469, t43766, t128, t43860);
        let t56176 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3052(t2435, t5048);
        let (t56179, t56181) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3053(t10356, t1469, t43776, t12305, t128);
        let t56183 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3054(t2435, t5053);
        let (t56184, t56185) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3055(t56183, t16739, t689);
        let t56187 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3056(t16743, t689);
        let t56189 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3057(t16734, t689);
    (t56165, t56167, t56172, t56174, t56176, t56179, t56181, t56183, t56184, t56185, t56187, t56189)
}
