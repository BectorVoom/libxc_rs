//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 690/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk690(t5: f64, t387: f64, t73: f64, t2249: f64, t3537: f64, t760: f64, t1: f64, t1212: f64, t332: f64, t395: f64, t1069: f64, t1074: f64, t2192: f64, t2195: f64, t247: f64, t330: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t4359 = t387 * t73;
    let t4360 = t4359 * t2249;
    let t4363 = t3537 * t760;
    let t4366 = t1212 * t1;
    let t4367 = t332 * t395;
    let t4377 = piecewise3(t6, 0.0_f64, 8.0_f64 / 27.0_f64 * t4363 * t1069 - 8.0_f64 / 9.0_f64 * t4366 * t4367 - 2.0_f64 / 9.0_f64 * t2192 * t1074 + 4.0_f64 / 3.0_f64 * t330 * t395 - 4.0_f64 * t2195 * t247);
    (t4359, t4360, t4363, t4366, t4367, t4377)
}
