//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1006/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1006(t11576: f64, t147: f64, t169: f64, t125: f64, t1482: f64, t619: f64, t11423: f64, t3081: f64, t11428: f64, t144: f64, t1461: f64, t1030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11577 = t11576 * t147;
    let t11578 = t169 * t11577;
    let t11579 = t1482 * t125;
    let t11580 = t11579 * t619;
    let t11581 = t11578 * t11580;
    let t11584 = t169 * t11423 * t3081;
    let t11586 = t11428 * t144;
    let t11587 = t1461 * t11586;
    let t11588 = t1030 * t11587;
    (t11577, t11578, t11579, t11580, t11581, t11584, t11587, t11588)
}
