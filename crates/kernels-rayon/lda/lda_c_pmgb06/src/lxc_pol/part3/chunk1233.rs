//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1233/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1233(t1309: f64, t769: f64, t395: f64, t4575: f64, t123: f64, t2822: f64, t868: f64, t14277: f64, t199: f64, t4435: f64, t722: f64, t10896: f64, t10903: f64, t10905: f64, t10925: f64, t10928: f64, t10931: f64, t10934: f64, t10937: f64, t10940: f64, t10943: f64, t10946: f64, t10949: f64, t11188: f64, t11695: f64, t305: f64, t726: f64, t81: f64) -> (f64, f64) {
    let t14656 = t1309 * t769;
    let t14663 = t395 * t4575;
    let t14666 = t123 * t2822 * t868;
    let t14669 = t123 * t14277 * t199;
    let t14672 = t123 * t722 * t4435;
    let t14694 = -0.31995040645307626_f64 * t14663 + 0.5188034422540342_f64 * t14666 + 0.5188034422540342_f64 * t14669 + 0.15917832887339686_f64 * t14672 + 0.05332506774217938_f64 * t81 * t11188 + 0.9598512193592288_f64 * t10903 - 3.839404877436915_f64 * t10905 + 0.053059442957798957_f64 * t10949 + 0.15917832887339686_f64 * t10931 - 0.42447554366239165_f64 * t10934 - 0.8489510873247833_f64 * t10937 - 0.031835665774679375_f64 * t123 * t305 * t11695 - 0.42447554366239165_f64 * t10940 + 1.5564103267621028_f64 * t10943 + 1.5564103267621028_f64 * t10946 - 0.10665013548435875_f64 * t10896 - 0.09550699732403813_f64 * t123 * t726 * t4435 + 0.053059442957798957_f64 * t10925 + 0.15917832887339686_f64 * t10928;
    (t14656, t14694)
}
