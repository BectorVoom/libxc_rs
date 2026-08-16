//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 387/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk387(t103: f64, t133: f64, t193: f64, t197: f64, t102: f64, t745: f64, t48: f64, t53: f64, t539: f64, t592: f64, t544: f64, t171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1923 = 1100.0_f64 / 81.0_f64 * t193 * t133 * t103 * t197;
    let t1924 = t745 * t102;
    let t1933 = 1.0_f64 / t48;
    let t1940 = 1.0_f64 / t53;
    let t1966 = 8.0_f64 * t539 * t592;
    let t1968 = 8.0_f64 * t544 * t592;
    let t1974 = t171 * t171;
    (t1923, t1924, t1933, t1940, t1966, t1968, t1974)
}
