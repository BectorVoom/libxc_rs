//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1121/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1121(t11038: f64, t967: f64, t7148: f64, t10888: f64, t10890: f64, t10893: f64, t10898: f64, t10913: f64, t10915: f64, t10922: f64, t10924: f64, t6969: f64, t7021: f64, t9008: f64, t9235: f64) -> (f64, f64, f64) {
    let t11039 = t11038 * t967;
    let t11041 = 0.51726012919273400301e3_f64 * t7148 * t11039;
    let t11056 = 0.264729375e1_f64 * t10888 - 0.3529725e1_f64 * t10890 - 0.17648625e1_f64 * t10893 + 0.3529725e1_f64 * t10915 - t7021 + 0.68863333333333333333e0_f64 * t6969 + 0.13772666666666666667e1_f64 * t9008 - t9235 - 0.516475e0_f64 * t10898 + 0.1549425e1_f64 * t10913 - 0.157790625e0_f64 * t10922 + 0.6311625e0_f64 * t10924;
    (t11039, t11041, t11056)
}
