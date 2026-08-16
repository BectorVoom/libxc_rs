//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2717/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2717(t1266: f64, t1271: f64, t1459: f64, t1778: f64, t19451: f64, t20098: f64, t20136: f64, t20143: f64, t20296: f64, t20698: f64, t22425: f64, t26114: f64, t26179: f64, t4026: f64, t4028: f64, t4037: f64, t510: f64, t5494: f64, t55943: f64, t6287: f64, t650: f64, t652: f64, t671: f64, t7458: f64, t75560: f64, t75701: f64) -> f64 {
    let t75762 = -2.0_f64 * t22425 * t652 * t671 - 2.0_f64 * t510 * t652 * t75701 - 6.0_f64 * t1266 * t20296 + t1271 * t20698 - 6.0_f64 * t1459 * t55943 - 6.0_f64 * t1459 * t75560 + 3.0_f64 * t1778 * t20098 - 6.0_f64 * t19451 * t4037 - 12.0_f64 * t20136 * t4028 - 6.0_f64 * t20143 * t7458 - t22425 * t650 - 6.0_f64 * t26114 * t5494 - 6.0_f64 * t26179 * t5494 - 3.0_f64 * t4026 * t6287;
    t75762
}
