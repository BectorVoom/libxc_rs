//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 543/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk543(t1133: f64, t4380: f64, t441: f64, t442: f64, t140: f64, t309: f64, t446: f64) -> (f64, f64, f64) {
    let t4381 = t1133 * t4380;
    let t4383 = t441 * t442;
    let t4385 = t446 * t309 * t140;
    let t4386 = t4383 * t4385;
    (t4381, t4383, t4386)
}
