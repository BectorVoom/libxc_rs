//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 877/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk877(t3284: f64, t8415: f64, t914: f64, t2849: f64, t6548: f64, t1221: f64, t2855: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8416 = t3284 * t8415;
    let t8417 = t914 * t8416;
    let t8420 = t2849 * t6548;
    let t8421 = t1221 * t8420;
    let t8422 = t914 * t8421;
    let t8425 = 1.0_f64 / t371 / t2855;
    (t8416, t8417, t8420, t8421, t8422, t8425)
}
