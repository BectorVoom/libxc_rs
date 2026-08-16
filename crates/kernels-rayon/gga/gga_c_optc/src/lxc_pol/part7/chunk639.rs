//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 639/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk639(t3204: f64, t438: f64, t894: f64, t1167: f64, t115: f64, t2770: f64, t426: f64, t123: f64, t3187: f64, t458: f64, t1724: f64, t3105: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3205 = t3204 * t438;
    let t3206 = t894 * t3205;
    let t3209 = t1167 * sigma2;
    let t3211 = t426 * t2770 * t115;
    let t3212 = t3209 * t3211;
    let t3213 = t3187 * t123;
    let t3214 = t458 * t3213;
    let t3217 = t1724 * t3211;
    let t3218 = t3105 * t123;
    (t3205, t3206, t3209, t3212, t3213, t3214, t3217, t3218)
}
