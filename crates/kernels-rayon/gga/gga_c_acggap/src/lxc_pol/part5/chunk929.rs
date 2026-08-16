//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 929/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk929(t14283: f64, t425: f64, t431: f64, t438: f64, t3243: f64, t390: f64, t996: f64, t1020: f64, t3237: f64, t1039: f64, t12295: f64, t383: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14284 = t14283 * t425;
    let t14286 = t14283 * t431;
    let t14288 = t14283 * t438;
    let t14292 = 0.12004725073059526352e-1_f64 * t3243 * t996 * t390;
    let t14297 = t3237 * t1020;
    let t14301 = 0.25724410870841842184e-2_f64 * t12295 * t383 * t1039;
    (t14284, t14286, t14288, t14292, t14297, t14301)
}
