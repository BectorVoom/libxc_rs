//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 995/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk995(t3382: f64, t4364: f64, t3372: f64, t5133: f64, t4326: f64, t14220: f64, t4916: f64, t4389: f64, t4393: f64, t4567: f64, t1165: f64, t3451: f64, t4183: f64, t4289: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16569 = t3382 * t4364;
    let t16575 = t3372 * t5133;
    let t16602 = t3372 * t4326;
    let t16608 = t14220 * t4916;
    let t16610 = t4389 * t4393;
    let t16612 = t4389 * t4567;
    let t16625 = t3451 * t1165 * t4289 * t4183;
    (t16569, t16575, t16602, t16608, t16610, t16612, t16625)
}
