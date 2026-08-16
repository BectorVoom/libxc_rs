//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 682/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk682(t12669: f64, t825: f64, t10007: f64, t935: f64, t9438: f64, t2610: f64, t3234: f64, t2365: f64, t2033: f64, t959: f64, t9817: f64, t10033: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12670 = t825 * t12669;
    let t12691 = t10007 * t935;
    let t12692 = t9438 * t12691;
    let t12693 = t825 * t12692;
    let t12695 = t2610 * t3234;
    let t12696 = t2365 * t12695;
    let t12697 = t2033 * t12696;
    let t12699 = t9817 * t959;
    let t12701 = t10033 * t959;
    (t12670, t12691, t12692, t12693, t12695, t12696, t12697, t12699, t12701)
}
