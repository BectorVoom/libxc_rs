//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 611/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk611(t11576: f64, t739: f64, t738: f64, t169: f64, t299: f64, t706: f64, t3645: f64, t702: f64, t3614: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11577 = t739 * t11576;
    let t11578 = t738 * t11577;
    let t11588 = t11576 * t169 * t299;
    let t11589 = t706 * t11588;
    let t11592 = t3645 * t702;
    let t11595 = t795 * t3614;
    (t11577, t11578, t11588, t11589, t11592, t11595)
}
