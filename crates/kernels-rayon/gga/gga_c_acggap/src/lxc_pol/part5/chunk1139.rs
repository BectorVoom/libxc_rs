//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1139/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1139(t1164: f64, t5679: f64, t12813: f64, t5732: f64, t3409: f64, t5612: f64, t513: f64, t943: f64, t157: f64, t1165: f64, t14373: f64, t1532: f64, t1748: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20417 = t1164 * t5679;
    let t20422 = t12813 * t5732;
    let t20430 = t3409 * t5612;
    let t20432 = t513 * t943;
    let t20433 = t20432 * t157;
    let t20441 = t14373 * t1165 * t1532 * t1748 * t864;
    (t20417, t20422, t20430, t20432, t20433, t20441)
}
