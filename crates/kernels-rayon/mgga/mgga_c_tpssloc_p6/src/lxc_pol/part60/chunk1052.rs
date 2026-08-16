//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1052/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1052(t127109: f64, t127111: f64, t128387: f64, t128393: f64, t128397: f64, t128401: f64, t128404: f64, t128406: f64, t128413: f64, t128415: f64, t19451: f64, t2075: f64, t2114: f64, t2165: f64, t27863: f64, t28959: f64, t29197: f64, t29214: f64, t29219: f64, t29486: f64, t33690: f64, t7266: f64, t7802: f64, t8835: f64) -> f64 {
    let t130342 = -2.0_f64 * t19451 * t8835 - t2075 * t29486 - t2114 * t29197 - 2.0_f64 * t2165 * t28959 - 4.0_f64 * t27863 * t7802 - 2.0_f64 * t29214 * t7266 - 4.0_f64 * t29219 * t7266 - 4.0_f64 * t33690 * t7802 - t127109 - t127111 - t128387 + t128393 + t128397 - t128401 - t128404 - t128406 - t128413 - t128415;
    t130342
}
