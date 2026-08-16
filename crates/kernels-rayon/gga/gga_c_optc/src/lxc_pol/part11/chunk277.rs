//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 277/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk277(t1085: f64, t406: f64, t1023: f64, t1049: f64, t414: f64) -> (f64, f64, f64, f64) {
    let t1086 = t406 * t1085;
    let t1088 = 0.301925e0_f64 * t1023;
    let t1091 = 0.82785e-1_f64 * t1049;
    let t1094 = 1.0_f64 / t414;
    (t1086, t1088, t1091, t1094)
}
