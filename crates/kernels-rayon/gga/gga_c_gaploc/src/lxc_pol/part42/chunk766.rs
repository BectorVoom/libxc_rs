//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 766/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk766(t2366: f64, t38271: f64, t3689: f64, t475: f64, t6508: f64, t12000: f64, t158: f64, t203: f64, t1: f64, t544: f64, t1359: f64, t12078: f64, t1397: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38272 = t2366 * t38271;
    let t38276 = t3689 * t475;
    let t38277 = t6508 * t38276;
    let t38281 = t2366 * t38276;
    let t38285 = t158 * t12000;
    let t38413 = t203 * t12000;
    let t38486 = t544 * t38285 * t1;
    let t38674 = t1359 * t3689;
    let t38770 = t1397 * t12078;
    (t38272, t38276, t38277, t38281, t38413, t38486, t38674, t38770)
}
