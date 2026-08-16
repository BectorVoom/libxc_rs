//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 846/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk846(t9707: f64, t9715: f64, t13111: f64, t13114: f64, t9721: f64, t6480: f64, t6484: f64, t6488: f64, t6492: f64, t6816: f64, t6823: f64, t6827: f64, t6840: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16344 = 0.73246220147012639764e-3_f64 * t9707;
    let t16345 = 24.0_f64 * t9715;
    let t16346 = 3.0_f64 * t13111;
    let t16347 = 0.54934665110259479823e-3_f64 * t13114;
    let t16348 = 24.0_f64 * t9721;
    let t16349 = t16344 + t6816 - t16345 - t6480 - t6484 + t6488 - t6823 + t6827 + t16346 - t16347 - t16348 + t6492 - t6840;
    (t16344, t16345, t16346, t16347, t16348, t16349)
}
