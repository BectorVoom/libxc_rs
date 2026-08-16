//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 531/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk531(t2479: f64, t2523: f64, t2521: f64, t2454: f64, t2457: f64, t2468: f64, t974: f64, t978: f64) -> (f64, f64, f64, f64, f64) {
    let t2524 = t2479 * t2523;
    let t2526 = 0.16081979498692535067e2_f64 * t2521 * t2524;
    let t2527 = 0.22831111111111111111e-1_f64 * t2454;
    let t2530 = t2527 - 0.34246666666666666666e-1_f64 * t2457 + 0.5137e-1_f64 * t2468;
    let t2533 = t974 * t978;
    (t2524, t2526, t2527, t2530, t2533)
}
