//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 683/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk683(t11135: f64, t3020: f64, t1771: f64, t926: f64, t3044: f64, t458: f64, t3047: f64, t14: f64, t7741: f64, t12: f64, t9: f64, t3053: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11136 = t3020 * t11135;
    let t11167 = t1771 * t926;
    let t11169 = t458 * t3044;
    let t11170 = 4.0_f64 / 27.0_f64 * t11169;
    let t11171 = t458 * t3047;
    let t11172 = 4.0_f64 / 9.0_f64 * t11171;
    let t11174 = 1.0_f64 / t14 / t7741;
    let t11175 = t12 * t11174;
    let t11176 = t9 * t11175;
    let t11177 = t11176 * t3053;
    (t11136, t11167, t11169, t11170, t11171, t11172, t11174, t11175, t11176, t11177)
}
