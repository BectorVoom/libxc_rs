//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 511/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk511(t1329: f64, t778: f64, t1333: f64, t2382: f64, t2399: f64, t1339: f64, t531: f64) -> (f64, f64, f64, f64) {
    let t3657 = t1329 * t778;
    let t3665 = t2382 * t1333;
    let t3681 = t2399 * t1333;
    let t3687 = t531 * t1339;
    (t3657, t3665, t3681, t3687)
}
