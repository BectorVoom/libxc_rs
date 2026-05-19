//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1233/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1233<F: Float>(t1309: F, t769: F, t395: F, t4575: F, t123: F, t2822: F, t868: F, t14277: F, t199: F, t4435: F, t722: F, t10896: F, t10903: F, t10905: F, t10925: F, t10928: F, t10931: F, t10934: F, t10937: F, t10940: F, t10943: F, t10946: F, t10949: F, t11188: F, t11695: F, t305: F, t726: F, t81: F) -> (F, F) {
    let t14656 = t1309 * t769;
    let t14663 = t395 * t4575;
    let t14666 = t123 * t2822 * t868;
    let t14669 = t123 * t14277 * t199;
    let t14672 = t123 * t722 * t4435;
    let t14694 = -F::cast_from(0.31995040645307626_f64) * t14663 + F::cast_from(0.5188034422540342_f64) * t14666 + F::cast_from(0.5188034422540342_f64) * t14669 + F::cast_from(0.15917832887339686_f64) * t14672 + F::cast_from(0.05332506774217938_f64) * t81 * t11188 + F::cast_from(0.9598512193592288_f64) * t10903 - F::cast_from(3.839404877436915_f64) * t10905 + F::cast_from(0.053059442957798957_f64) * t10949 + F::cast_from(0.15917832887339686_f64) * t10931 - F::cast_from(0.42447554366239165_f64) * t10934 - F::cast_from(0.8489510873247833_f64) * t10937 - F::cast_from(0.031835665774679375_f64) * t123 * t305 * t11695 - F::cast_from(0.42447554366239165_f64) * t10940 + F::cast_from(1.5564103267621028_f64) * t10943 + F::cast_from(1.5564103267621028_f64) * t10946 - F::cast_from(0.10665013548435875_f64) * t10896 - F::cast_from(0.09550699732403813_f64) * t123 * t726 * t4435 + F::cast_from(0.053059442957798957_f64) * t10925 + F::cast_from(0.15917832887339686_f64) * t10928;
    (t14656, t14694)
}
