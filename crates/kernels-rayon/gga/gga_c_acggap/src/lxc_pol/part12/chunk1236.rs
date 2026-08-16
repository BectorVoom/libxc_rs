//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1236/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1236(t38092: f64, t7963: f64, t7965: f64, t4210: f64, t7942: f64, t2385: f64, t323: f64, t851: f64, t7990: f64, t9154: f64, t862: f64, t865: f64) -> (f64, f64, f64, f64, f64) {
    let t38280 = 0.17347256376410398924e1_f64 * t7963 * t38092 * t7965;
    let t38283 = 0.17347256376410398924e1_f64 * t7942 * t38092 * t4210;
    let t38285 = t851 * t2385 * t323;
    let t38293 = 0.34694512752820797848e1_f64 * t7990 * t9154;
    let t38309 = t862 * t2385 * t865;
    (t38280, t38283, t38285, t38293, t38309)
}
