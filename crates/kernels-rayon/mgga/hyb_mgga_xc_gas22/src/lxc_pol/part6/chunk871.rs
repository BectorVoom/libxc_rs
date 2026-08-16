//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 871/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk871(t2639: f64, t7238: f64, t7253: f64, t1110: f64, t2643: f64, t2649: f64, t1030: f64, t1884: f64, t1048: f64, t222: f64, t2711: f64, t2714: f64, t567: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7255 = t7253 * t7238 * t2639;
    let t7257 = 0.10389515463408878255e3_f64 * t1110 * t7255;
    let t7258 = t2643 * t2649;
    let t7260 = t1884 * t1030;
    let t7263 = 0.71233333333333333332e-1_f64 * t222 * t7260 * t1048;
    let t7267 = 0.10685e0_f64 * t222 * t567 * t2711 * t2714;
    (t7255, t7257, t7258, t7260, t7263, t7267)
}
