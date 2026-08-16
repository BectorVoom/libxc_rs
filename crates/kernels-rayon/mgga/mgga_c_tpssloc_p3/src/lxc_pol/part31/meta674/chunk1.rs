//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2037/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2037(t12725: f64, t1774: f64, t19451: f64, t19456: f64, t20100: f64, t20136: f64, t20143: f64, t22574: f64, t23938: f64, t26977: f64, t27147: f64, t27150: f64, t27163: f64, t27170: f64, t27226: f64, t28002: f64, t28821: f64, t28830: f64, t29247: f64, t32193: f64, t4028: f64, t5494: f64, t6287: f64, t652: f64, t7042: f64, t7056: f64, t7057: f64, t7061: f64, t7220: f64, t7458: f64, t7796: f64, t7802: f64, t83886: f64) -> f64 {
    let t103070 = -4.0_f64 * t7458 * t27150 - 6.0_f64 * t22574 * t32193 * t28830 - 6.0_f64 * t83886 * t29247 - 4.0_f64 * t28002 * t7057 - 4.0_f64 * t12725 * t7796 - 4.0_f64 * t19456 * t7796 - 4.0_f64 * t4028 * t27163 - 4.0_f64 * t7042 * t20136 - 2.0_f64 * t19451 * t7061 - 2.0_f64 * t652 * t6287 * t7056 - 2.0_f64 * t7042 * t20100 - t28821 * t7220 - 4.0_f64 * t652 * t1774 * t27170 - 2.0_f64 * t23938 * t5494 - 2.0_f64 * t26977 * t5494 - 2.0_f64 * t7042 * t20143 - 4.0_f64 * t4028 * t27147 - 4.0_f64 * t19456 * t7802 - 4.0_f64 * t4028 * t27226;
    t103070
}
