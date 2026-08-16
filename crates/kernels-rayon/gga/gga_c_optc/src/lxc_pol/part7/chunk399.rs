//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 399/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk399(t43: f64, t50: f64, t48: f64, t1885: f64, t1891: f64, t607: f64, t53: f64, t1897: f64, t1900: f64, t611: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1933 = 1.0_f64 / t48;
    let t1939 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t1933 * t1885 + 2.0_f64 / 3.0_f64 * t607 * t1891);
    let t1940 = 1.0_f64 / t53;
    let t1946 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t1940 * t1897 + 2.0_f64 / 3.0_f64 * t611 * t1900);
    let t1948 = t1939 / 2.0_f64 + t1946 / 2.0_f64;
    (t1933, t1940, t1948)
}
