//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1046/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1046(t4928: f64, t500: f64, t1476: f64, t170: f64, t1475: f64, t1697: f64, t475: f64, t474: f64, t16190: f64, t49: f64, t55: f64, t204: f64, t47: f64, t5401: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16202 = t4928 * t500;
    let t16204 = t1476 * t170;
    let t16205 = t1475 * t16204;
    let t16207 = t475 * t1697;
    let t16208 = t474 * t16207;
    let t16210 = t49 * t16190;
    let t16212 = f64::powf(t55, -0.25e1_f64);
    let t16215 = t16212 * t47 * t5401 * t204;
    (t16202, t16204, t16205, t16207, t16208, t16210, t16215)
}
