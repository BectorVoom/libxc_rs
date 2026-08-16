//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 401/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk401(t1923: f64, t1926: f64, t1928: f64, t193: f64, t1949: f64, t195: f64, t197: f64, t750: f64, t201: f64, t5: f64, t743: f64) -> (f64, f64, f64) {
    let t1953 = -t1923 + 400.0_f64 / 27.0_f64 * t1926 - 25.0_f64 / 9.0_f64 * t193 * t195 * t1928 * t197 - 25.0_f64 / 9.0_f64 * t193 * t750 * t1949;
    let t1955 = t5 * t1953 * t201;
    let t1956 = t743 * t1955;
    (t1953, t1955, t1956)
}
