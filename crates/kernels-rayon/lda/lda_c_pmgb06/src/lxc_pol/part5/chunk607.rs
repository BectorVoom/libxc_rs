//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 607/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk607(t4232: f64, t4354: f64, t1233: f64, t4230: f64, t387: f64, t73: f64, t2249: f64, t3537: f64, t760: f64, t1: f64, t1212: f64, t3548: f64, t764: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4355 = t4232 * t4354;
    let t4358 = t1233 * t4230;
    let t4359 = t387 * t73;
    let t4360 = t4359 * t2249;
    let t4363 = t3537 * t760;
    let t4366 = t1212 * t1;
    let t4378 = t3548 * t764;
    (t4355, t4358, t4359, t4360, t4363, t4366, t4378)
}
