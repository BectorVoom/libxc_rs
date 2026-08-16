//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1188/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1188(t1181: f64, t5814: f64, t7351: f64, t7564: f64, t1854: f64, t30148: f64, t30159: f64, t7586: f64, t1967: f64, t9691: f64, t30780: f64, t38956: f64) -> (f64, f64, f64, f64) {
    let t40561 = t7564 * t1181 * t7351 * t5814;
    let t40565 = t30159 * t7586 * t30148 * t1854;
    let t40567 = t1967 * t9691;
    let t40569 = t30780 * t38956;
    (t40561, t40565, t40567, t40569)
}
