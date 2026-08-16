//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 451/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk451(t143: f64, t2054: f64, t39: f64, t2028: f64, t2052: f64, t2022: f64, t699: f64, t2002: f64, t702: f64, t2047: f64, t2048: f64, t572: f64, t147: f64, t168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t145 = 0.135e1_f64 < t143;
    let t2055 = t39 * t2054;
    let t2057 = t2052 * t2055 * t2028;
    let t2060 = t39 * t2022;
    let t2062 = t699 * t2060 * t2028;
    let t2066 = t699 * t702 * t2002;
    let t2069 = t2047 + t2048 / 81.0_f64 - t572 * t2057 / 81.0_f64 + t572 * t2062 / 27.0_f64 - t572 * t2066 / 54.0_f64;
    let t2070 = piecewise3(t145, t2069, 0.0_f64);
    let t2098 = 1.0_f64 / t168 / t147;
    (t2055, t2057, t2060, t2062, t2066, t2069, t2070, t2098)
}
