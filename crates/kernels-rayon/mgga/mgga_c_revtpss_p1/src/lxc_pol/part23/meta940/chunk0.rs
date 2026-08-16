//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3087/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3087(t20365: f64, t5079: f64, t16862: f64, t6449: f64, t20337: f64, t5087: f64, t1134: f64, t24312: f64, t3407: f64, t141: f64, t3417: f64, t81177: f64) -> (f64, f64, f64, f64, f64) {
    let t81523 = t20365 * t5079;
    let t81525 = t16862 * t6449;
    let t81527 = t5087 * t20337;
    let t81530 = t3407 * t24312 * t1134;
    let t81533 = t141 * t3417 * t81177;
    (t81523, t81525, t81527, t81530, t81533)
}
