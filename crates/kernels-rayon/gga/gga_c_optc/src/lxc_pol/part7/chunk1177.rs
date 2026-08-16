//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1177/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1177(t2269: f64, t7467: f64, t2640: f64, t7484: f64, t2639: f64, t2730: f64, t2643: f64, t7299: f64, t2641: f64, t7373: f64, t7380: f64, t769: f64) -> (f64, f64, f64, f64, f64) {
    let t24416 = t7467 * t2269;
    let t24418 = t2640 * t24416 * t7484;
    let t24420 = t2730 * t2639;
    let t24427 = t2643 * t7299;
    let t24431 = t2641 * t7373;
    let t24432 = t7380 * t769;
    (t24418, t24420, t24427, t24431, t24432)
}
