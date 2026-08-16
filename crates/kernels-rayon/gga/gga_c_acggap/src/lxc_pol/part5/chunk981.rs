//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 981/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk981(t3379: f64, t5272: f64, t3375: f64, t5129: f64, t5133: f64, t3372: f64, t4987: f64, t13502: f64, t542: f64, t1569: f64, t3237: f64, t1181: f64, t12816: f64, t3361: f64, t4643: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16117 = t3379 * t5272;
    let t16123 = t3375 * t5129;
    let t16125 = t3375 * t5133;
    let t16127 = t3372 * t4987;
    let t16141 = t13502 * t542;
    let t16143 = t3237 * t1569;
    let t16160 = t3361 * t1181 * t4643 * t12816;
    (t16117, t16123, t16125, t16127, t16141, t16143, t16160)
}
