//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 845/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk845(t166: f64, t117: f64, t3033: f64, t130: f64, t972: f64, t182: f64, t851: f64, t1015: f64, t173: f64, t157: f64, t406: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13461 = t166 * t166;
    let t13462 = 1.0_f64 / t13461;
    let t13483 = 1.0_f64 / t3033 / t117;
    let t13716 = t130 * t972;
    let t14046 = t851 * t182;
    let t14423 = 1.0_f64 / t1015 / t173;
    let t14575 = t879 * t406 * t157;
    (t13462, t13483, t13716, t14046, t14423, t14575)
}
