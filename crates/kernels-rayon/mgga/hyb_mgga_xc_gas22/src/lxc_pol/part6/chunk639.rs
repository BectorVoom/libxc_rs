//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 639/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk639(t1193: f64, t69: f64, t3029: f64, t608: f64, t1941: f64, t612: f64, t3002: f64, t3032: f64, t3035: f64, t3037: f64, t3040: f64, t3042: f64, t3045: f64, t3047: f64, t3050: f64, t3052: f64, t3055: f64, t565: f64, t584: f64) -> (f64, f64, f64) {
    let t3057 = t69 * t1193;
    let t3060 = t608 * t3029;
    let t3062 = t1941 * t1193;
    let t3065 = t612 * t3029;
    let t3067 = t3002 * t584 / 6.0_f64 - t565 * t3029 / 18.0_f64 - t3032 * t584 / 48.0_f64 + t3035 / 240.0_f64 + t3037 * t584 / 640.0_f64 - t3040 / 4480.0_f64 - t3042 * t584 / 11520.0_f64 + t3045 / 103680.0_f64 + t3047 * t584 / 258048.0_f64 - t3050 / 2838528.0_f64 - t3052 * t584 / 6881280.0_f64 + t3055 / 89456640.0_f64 + t3057 * t584 / 0.21233664e9_f64 - t3060 / 0.31850496e10_f64 - t3062 * t584 / 0.74317824e10_f64 + t3065 / 0.1263403008e12_f64;
    (t3057, t3062, t3067)
}
