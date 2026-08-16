//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 961/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk961(t3237: f64, t9189: f64, t3234: f64, t3151: f64, t9044: f64, t894: f64, t2860: f64, t3236: f64, t3235: f64, t3146: f64, t3087: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9190 = t9189 * t3237;
    let t9191 = t3234 * t9190;
    let t9193 = t3151 * t9044;
    let t9194 = t894 * t9193;
    let t9197 = t2860 * t3236;
    let t9198 = t3235 * t9197;
    let t9201 = t3146 * t9044;
    let t9202 = t894 * t9201;
    let t9205 = t3087 * t9044;
    let t9206 = t914 * t9205;
    (t9191, t9193, t9194, t9197, t9198, t9201, t9202, t9205, t9206)
}
