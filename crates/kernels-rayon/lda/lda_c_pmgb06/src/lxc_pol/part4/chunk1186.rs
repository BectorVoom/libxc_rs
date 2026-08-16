//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1186/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1186(t12393: f64, t13566: f64, t13619: f64, t13621: f64, t13633: f64, t13635: f64, t13637: f64, t13639: f64, t13644: f64, t15324: f64, t15405: f64, t15407: f64, t15413: f64, t15548: f64) -> f64 {
    let t15641 = 0.026660493827160493_f64 * t15405 + 0.3519185185185185_f64 * t15407 - 0.03999074074074074_f64 * t15413 - 0.023703703703703703_f64 * t15548 * t13566 * t15324 + 0.05925925925925926_f64 * t13619 - 0.009876543209876543_f64 * t13621 + 0.002962962962962963_f64 * t13633 + 0.003950617283950617_f64 * t13635 + 0.011851851851851851_f64 * t13637 - 0.017777777777777778_f64 * t13639 + 0.05333333333333334_f64 * t13644 - 0.09597777777777777_f64 * t12393;
    t15641
}
