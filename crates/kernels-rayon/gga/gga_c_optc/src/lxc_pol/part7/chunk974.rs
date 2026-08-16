//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 974/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk974(t172: f64, t1928: f64, t3314: f64, t622: f64, t3313: f64, t176: f64, t729: f64, t3315: f64, t108: f64, t616: f64, t110: f64, t131: f64, t2020: f64) -> (f64, f64, f64, f64, f64) {
    let t9361 = t1928 * t172;
    let t9411 = t3314 * t622;
    let t9412 = t3313 * t9411;
    let t9415 = t176 * t729;
    let t9416 = t9415 * t3315;
    let t9546 = t616 * t108;
    let t9547 = t9546 * t110;
    let t9548 = t3313 * t9547;
    let t9598 = t2020 * t131;
    (t9361, t9412, t9416, t9548, t9598)
}
