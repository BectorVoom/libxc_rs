//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 454/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk454(t2205: f64, t2984: f64, t446: f64, t558: f64, t925: f64, t1969: f64, t2993: f64, t569: f64, t18: f64, t519: f64, t3000: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3320 = t2205 * t2984;
    let t3321 = t446 * t3320;
    let t3323 = t925 * t558;
    let t3324 = t1969 * t3323;
    let t3325 = t446 * t3324;
    let t3327 = t569 * t2993;
    let t3328 = t446 * t3327;
    let t3330 = t519 * t18;
    let t3332 = t89 * t3000 * t3330;
    (t3320, t3321, t3323, t3324, t3325, t3327, t3328, t3330, t3332)
}
