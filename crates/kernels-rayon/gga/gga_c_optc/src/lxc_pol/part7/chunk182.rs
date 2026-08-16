//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 182/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk182(t321: f64, t458: f64, t439: f64, t442: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t459 = t321 * t458;
    let t462 = t439 * t439;
    let t463 = 1.0_f64 / t462;
    let t464 = t463 * t442;
    let t465 = t464 * t446;
    (t459, t462, t463, t464, t465)
}
