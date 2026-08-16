//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1078/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1078(t3378: f64, t4173: f64, t13259: f64, t1630: f64, t1160: f64, t1539: f64, t18973: f64, t4166: f64, t4210: f64, t4146: f64, t16020: f64, t1629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19237 = t3378 * t4173;
    let t19240 = t13259 * t1630;
    let t19243 = t1160 * t18973 * t1539;
    let t19246 = t1160 * t4166 * t4210;
    let t19249 = t1160 * t4146 * t4210;
    let t19252 = t1160 * t1629 * t16020;
    (t19237, t19240, t19243, t19246, t19249, t19252)
}
