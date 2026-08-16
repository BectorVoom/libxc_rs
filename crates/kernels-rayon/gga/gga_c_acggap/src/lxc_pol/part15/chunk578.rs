//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 578/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk578(t368: f64, t398: f64, t5087: f64, t384: f64, t3476: f64, t527: f64, t513: f64, t864: f64, t1095: f64, t1036: f64, t1032: f64, t1434: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5089 = t398 * t368 * t5087;
    let t5090 = t384 * t5089;
    let t5092 = t3476 * t527;
    let t5099 = t513 * t864;
    let t5101 = t398 * t1095 * t5099;
    let t5102 = t1036 * t5101;
    let t5104 = t1032 * t1434;
    (t5089, t5090, t5092, t5099, t5101, t5102, t5104)
}
