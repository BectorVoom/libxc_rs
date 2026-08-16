//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1178/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1178(t103: f64, t11991: f64, t12547: f64, t12558: f64, t12584: f64, t12588: f64, t12594: f64, t12599: f64, t12605: f64, t12864: f64, t13330: f64, t13332: f64, t13335: f64, t13337: f64, t13340: f64, t13565: f64, t14073: f64, t14078: f64, t14080: f64, t14082: f64, t14106: f64, t14110: f64, t1576: f64, t2060: f64, t3358: f64, t525: f64, t9967: f64) -> f64 {
    let t14115 = -0.08_f64 * t103 * t1576 * t11991 + 0.08_f64 * t14073 + 0.16_f64 * t103 * t525 * t12864 - 0.02666666666666667_f64 * t14078 + 0.005925925925925926_f64 * t14080 - 0.057777777777777775_f64 * t14082 - 0.0022222222222222222_f64 * t103 * t1576 * t12584 + 0.013333333333333334_f64 * t2060 * t1576 * t12588 - 0.006913580246913581_f64 * t103 * t9967 * t12594 + 0.017777777777777778_f64 * t2060 * t3358 * t12599 + 0.013333333333333334_f64 * t103 * t525 * t12605 - 0.08_f64 * t2060 * t525 * t12558 - 0.8638_f64 * t13330 + 0.21595_f64 * t13332 + 0.8638_f64 * t13335 - 0.5278777777777778_f64 * t13337 - 0.12_f64 * t13565 * t14106 * t12547 - 0.008888888888888889_f64 * t13565 * t14110 * t12547 + 0.47988888888888886_f64 * t13340;
    t14115
}
