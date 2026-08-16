//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 448/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk448(t1008: f64, t2246: f64, t1007: f64, t23: f64, t6: f64, t1014: f64, t287: f64) -> (f64, f64, f64) {
    let t2247 = t1008 * t2246;
    let t2248 = t1007 * t2247;
    let t2251 = t6 * t23;
    let t2253 = t2251 * t287 * t1014;
    (t2247, t2248, t2253)
}
