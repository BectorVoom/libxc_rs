//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 852/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk852(t16323: f64, t5: f64, t6879: f64, t675: f64, t2024: f64, t127: f64, t16411: f64, t3519: f64, t16419: f64, t3491: f64, t16376: f64, t696: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16477 = t5 * t16323;
    let t16478 = t16477 * t6879;
    let t16479 = t675 * t16478;
    let t16482 = t16477 * t2024;
    let t16483 = t675 * t16482;
    let t16486 = t16477 * t127;
    let t16487 = t675 * t16486;
    let t16490 = t3519 * t16411;
    let t16493 = t3491 * t16419;
    let t16496 = t696 * t16376;
    (t16479, t16483, t16487, t16490, t16493, t16496)
}
