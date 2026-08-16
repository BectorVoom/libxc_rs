//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1083/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1083(t1014: f64, t23471: f64, t287: f64, t1010: f64, t2253: f64, t7314: f64, t1006: f64, t8378: f64, t2317: f64, t2325: f64, t7230: f64, t7234: f64) -> (f64, f64, f64, f64, f64) {
    let t23473 = t23471 * t287 * t1014;
    let t23474 = t1010 * t23473;
    let t23476 = t7314 * t2253;
    let t23481 = t1006 * t8378;
    let t23485 = t2325 * t2317;
    let t23490 = t7230 * t7234;
    (t23474, t23476, t23481, t23485, t23490)
}
