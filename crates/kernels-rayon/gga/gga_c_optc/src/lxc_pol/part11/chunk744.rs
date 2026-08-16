//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 744/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk744(t1994: f64, t3386: f64, t3314: f64, t622: f64, t3313: f64, t176: f64, t729: f64, t3315: f64, t1319: f64, t6680: f64, t188: f64, t1990: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9404 = t3386 * t1994;
    let t9411 = t3314 * t622;
    let t9412 = t3313 * t9411;
    let t9415 = t176 * t729;
    let t9416 = t9415 * t3315;
    let t9430 = t6680 * t1319;
    let t9431 = t188 * t9430;
    let t9477 = t3386 * t1990;
    (t9404, t9412, t9416, t9430, t9431, t9477)
}
