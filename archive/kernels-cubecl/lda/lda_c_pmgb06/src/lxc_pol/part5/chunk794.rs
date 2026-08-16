//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 794/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk794<F: Float>(t123: F, t199: F, t2415: F, t2422: F, t305: F, t4252: F, t4283: F, t4427: F, t4431: F, t4472: F, t7115: F, t7126: F, t7135: F, t7145: F, t7365: F, t7375: F, t7425: F, t81: F, t868: F, t912: F) -> F {
    let t7428 = t4252 - F::cast_from(0.42447554366239165_f64) * t4431 - F::cast_from(0.42447554366239165_f64) * t4427 + F::cast_from(0.15917832887339686_f64) * t7115 + F::cast_from(0.3183566577467937_f64) * t7126 + F::cast_from(0.15917832887339686_f64) * t7135 - F::cast_from(0.031835665774679375_f64) * t123 * t7365 * t199 - F::cast_from(0.09550699732403813_f64) * t123 * t2415 * t868 - F::cast_from(0.09550699732403813_f64) * t123 * t912 * t2422 - F::cast_from(0.031835665774679375_f64) * t123 * t305 * t7375 - t4283 + F::cast_from(0.9598512193592288_f64) * t4472 - F::cast_from(0.31995040645307626_f64) * t7145 + F::cast_from(0.05332506774217938_f64) * t81 * t7425;
    t7428
}
