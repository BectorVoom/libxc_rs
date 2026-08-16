//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1263/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1263(t123: f64, t722: f64, t7375: f64, t7113: f64, t868: f64, t2281: f64, t2422: f64, t10905: f64, t10943: f64, t10946: f64, t125: f64, t14666: f64, t14669: f64, t14697: f64, t1808: f64, t18954: f64, t18969: f64, t18979: f64, t19130: f64, t199: f64, t21827: f64, t2285: f64, t2415: f64, t566: f64, t6939: f64, t7117: f64, t7365: f64, t81: f64, t912: f64) -> f64 {
    let t22177 = t123 * t722 * t7375;
    let t22197 = t123 * t7113 * t868;
    let t22200 = t123 * t2281 * t2422;
    let t22214 = 0.053059442957798957_f64 * t22177 - 0.09550699732403813_f64 * t123 * t912 * t6939 - 0.09550699732403813_f64 * t123 * t2285 * t2422 + 0.9598512193592288_f64 * t18954 - 0.031835665774679375_f64 * t123 * t125 * t19130 * t199 - 0.031835665774679375_f64 * t123 * t7365 * t566 - 0.09550699732403813_f64 * t123 * t7117 * t868 + 0.15917832887339686_f64 * t22197 + 0.15917832887339686_f64 * t22200 + 1.5564103267621028_f64 * t14666 + 1.5564103267621028_f64 * t14669 + 0.15917832887339686_f64 * t18969 + 0.05332506774217938_f64 * t81 * t21827 - 0.09550699732403813_f64 * t123 * t2415 * t1808 - 1.279801625812305_f64 * t10905 + 0.5188034422540342_f64 * t10943 + 0.5188034422540342_f64 * t10946 - 0.31995040645307626_f64 * t18979 - t14697;
    t22214
}
