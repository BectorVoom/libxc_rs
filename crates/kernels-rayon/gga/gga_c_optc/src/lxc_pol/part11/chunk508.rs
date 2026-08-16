//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 508/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk508(t1384: f64, t888: f64, t874: f64, t1382: f64, t2641: f64, t1326: f64, t522: f64) -> (f64, f64, f64) {
    let t3631 = t888 * t1384;
    let t3632 = t874 * t3631;
    let t3634 = t2641 * t1382;
    let t3640 = t522 * t1326;
    (t3632, t3634, t3640)
}
