//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2623;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta748(t47060: f64, t13581: f64, t72: f64, t757: f64, t47073: f64, t5635: f64, t9586: f64, t5571: f64, t9425: f64, t47078: f64, t9318: f64, t1857: f64, t9342: f64, t39807: f64, t39813: f64, t47059: f64, t47063: f64, t47067: f64, t47070: f64, t47072: f64, t47076: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48275, t48278, t48279, t48281, t48283, t48284, t48286, t48287) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2623(t47060, t13581, t72, t757, t47073, t5635, t9586, t5571, t9425, t47078, t9318, t1857, t9342);
        let (t48288, t48289) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2624(t48287, t39807, t39813, t47059, t47063, t47067, t47070, t47072, t47076, t48275, t48278, t48279, t48281, t48283, t48284, t48286);
    (t48275, t48278, t48279, t48281, t48283, t48284, t48286, t48288, t48289)
}
