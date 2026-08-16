//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 860/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk860(t348: f64, t355: f64, t6966: f64, t345: f64, t238: f64, t353: f64, t6611: f64, t2213: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7009 = 1.0_f64 / t348 / t355 / 4.0_f64;
    let t7016 = 28.0_f64 / 27.0_f64 * t6966;
    let t7021 = 0.16068111111111111111e1_f64 * t6966;
    let t7025 = 1.0_f64/pow_3_2(t345);
    let t7034 = t238 * t6611 * t353;
    let t7035 = 0.46308888888888888888e0_f64 * t7034;
    let t7037 = t238 * t2213 * t963;
    (t7009, t7016, t7021, t7025, t7034, t7035, t7037)
}
