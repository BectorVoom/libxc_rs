//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1078/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1078(t3073: f64, t831: f64, t132: f64, t435: f64, t4681: f64, t1842: f64, t642: f64, t5375: f64, t591: f64, t4111: f64, t5378: f64, t5382: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12278 = t831 * t3073;
    let t12281 = t132 * t435 * t4681;
    let t12294 = 48.0_f64 * t1842 * t642;
    let t12304 = t5375 * t591;
    let t12306 = t5378 * t4111;
    let t12307 = (2e-21_f64 as f64) * t12306;
    let t12308 = t5382 * t591;
    (t12278, t12281, t12294, t12304, t12307, t12308)
}
