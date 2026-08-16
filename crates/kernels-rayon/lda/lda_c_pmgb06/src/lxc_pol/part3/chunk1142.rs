//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1142/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1142(t12368: f64, t4913: f64, t5028: f64, t405: f64, t5025: f64, t2057: f64, t955: f64, t2054: f64, t103: f64, t12156: f64, t12161: f64, t12165: f64, t12364: f64, t12366: f64, t12371: f64, t12374: f64, t12377: f64, t12380: f64, t12382: f64, t12384: f64, t12387: f64, t2060: f64, t3404: f64, t473: f64, t9693: f64, t9715: f64, t9719: f64) -> f64 {
    let t13595 = 0.03199259259259259_f64 * t12368;
    let t13602 = t4913 * t5028;
    let t13604 = t405 * t5025;
    let t13619 = t955 * t2057;
    let t13621 = t955 * t2054;
    let t13623 = 0.21595_f64 * t12364 + 0.09597777777777777_f64 * t12366 - t13595 - 0.023994444444444443_f64 * t12371 - 0.14396666666666666_f64 * t12374 - 0.10664197530864197_f64 * t12377 - 0.23994444444444443_f64 * t12380 + 0.07198333333333333_f64 * t12384 + 0.4319_f64 * t12387 + t9715 - 0.3466666666666667_f64 * t13602 - 0.02666666666666667_f64 * t13604 + 0.02666666666666667_f64 * t9719 - 0.006913580246913581_f64 * t103 * t9693 * t12156 - 0.017777777777777778_f64 * t2060 * t3404 * t12161 + 0.013333333333333334_f64 * t103 * t473 * t12382 + 0.08_f64 * t2060 * t473 * t12165 + 0.044444444444444446_f64 * t13619 - 0.007407407407407408_f64 * t13621;
    t13623
}
