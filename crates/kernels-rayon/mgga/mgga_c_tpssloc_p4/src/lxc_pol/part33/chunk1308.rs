//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1308/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1308(t28827: f64, t7685: f64, t23035: f64, t25224: f64, t28298: f64, t20756: f64, t6553: f64, t6554: f64, t81984: f64, t1527: f64, t22986: f64, t23270: f64, t98253: f64) -> (f64, f64, f64, f64) {
    let t105213 = 18.0_f64 * t7685 * t28827;
    let t105223 = t23035 * t25224 * t28298;
    let t105232 = t81984 * t6553 * t6554 * t20756;
    let t105240 = t22986 * t23270 * t98253 * t1527;
    (t105213, t105223, t105232, t105240)
}
