//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 775/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk775(t2667: f64, t946: f64, t312: f64, t9: f64, t2670: f64, t2674: f64, t2668: f64, t2679: f64, t2678: f64, t2574: f64, t858: f64, t2579: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7430 = t946 * t2667;
    let t7433 = t9 * t312;
    let t7434 = t7433 * t2670;
    let t7435 = t7434 * t2674;
    let t7436 = t2668 * t7435;
    let t7438 = t7434 * t2679;
    let t7439 = t2678 * t7438;
    let t7441 = t2574 * t858;
    let t7443 = t854 * t2579;
    (t7430, t7433, t7435, t7436, t7438, t7439, t7441, t7443)
}
