//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1180/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1180(t1096: f64, t1110: f64, t21846: f64, t7253: f64, t21837: f64, t2710: f64, t2727: f64, t441: f64, t7443: f64, t1055: f64, t25: f64, t12: f64, t20626: f64, t222: f64, t442: f64) -> (f64, f64, f64, f64) {
    let t21850 = 0.14035736694323150897e2_f64 * t1110 * t7253 * t21846 * t1096;
    let t21856 = 0.62071215503128080361e4_f64 * t441 / t2727 / t2710 * t21837 * t7443;
    let t21862 = 1.0_f64 / t25 / t1055;
    let t21864 = 1.0_f64 / t442 / t20626 * t12 * t21862 * t222 / 48.0_f64;
    (t21850, t21856, t21862, t21864)
}
