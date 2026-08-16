//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 794/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk794(t123: f64, t199: f64, t2415: f64, t2422: f64, t305: f64, t4252: f64, t4283: f64, t4427: f64, t4431: f64, t4472: f64, t7115: f64, t7126: f64, t7135: f64, t7145: f64, t7365: f64, t7375: f64, t7425: f64, t81: f64, t868: f64, t912: f64) -> f64 {
    let t7428 = t4252 - 0.42447554366239165_f64 * t4431 - 0.42447554366239165_f64 * t4427 + 0.15917832887339686_f64 * t7115 + 0.3183566577467937_f64 * t7126 + 0.15917832887339686_f64 * t7135 - 0.031835665774679375_f64 * t123 * t7365 * t199 - 0.09550699732403813_f64 * t123 * t2415 * t868 - 0.09550699732403813_f64 * t123 * t912 * t2422 - 0.031835665774679375_f64 * t123 * t305 * t7375 - t4283 + 0.9598512193592288_f64 * t4472 - 0.31995040645307626_f64 * t7145 + 0.05332506774217938_f64 * t81 * t7425;
    t7428
}
