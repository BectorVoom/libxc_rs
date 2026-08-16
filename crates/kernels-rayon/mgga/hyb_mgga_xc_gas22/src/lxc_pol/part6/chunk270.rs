//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 270/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk270(t899: f64, t902: f64, t120: f64, t307: f64, t328: f64, t332: f64, t319: f64, t97: f64, t99: f64, t315: f64, t324: f64, t122: f64, t331: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t903 = t899 * t902;
    let t907 = t328 * t307 * t120;
    let t908 = t907 * t332;
    let t909 = t319 * t97;
    let t911 = 1.0_f64 / t99 / t909;
    let t912 = t315 * t911;
    let t913 = t324 * tau0;
    let t914 = t912 * t913;
    let t918 = 1.0_f64 / t331 / t122;
    (t903, t907, t908, t909, t913, t914, t918)
}
