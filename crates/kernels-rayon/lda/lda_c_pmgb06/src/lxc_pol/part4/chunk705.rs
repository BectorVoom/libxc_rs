//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 705/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk705(t754: f64, t927: f64, t97: f64, t374: f64, t769: f64, t4232: f64, t1233: f64, t4230: f64, t387: f64, t73: f64) -> (f64, f64, f64, f64, f64) {
    let t4351 = t927 * t754 * t97;
    let t4354 = t769 * t374;
    let t4355 = t4232 * t4354;
    let t4358 = t1233 * t4230;
    let t4359 = t387 * t73;
    (t4351, t4354, t4355, t4358, t4359)
}
