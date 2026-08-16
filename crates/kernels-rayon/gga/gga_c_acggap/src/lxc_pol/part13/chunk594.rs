//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 594/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk594(t1140: f64, t1526: f64, t509: f64, t987: f64, t1165: f64, t1532: f64, t4162: f64, t1163: f64, t530: f64, t945: f64, t535: f64, t1181: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4368 = 7.0_f64 / 144.0_f64 * t1140 * t1526;
    let t4369 = t987 * t509;
    let t4372 = t1165 * t1532 * t4162;
    let t4373 = t1163 * t4372;
    let t4376 = t1165 * t530 * t945;
    let t4379 = t535 * t945;
    let t4380 = t1181 * t4379;
    (t4368, t4369, t4372, t4373, t4376, t4380)
}
