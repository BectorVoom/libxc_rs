//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1086/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1086<F: Float>(t1447: F, t6114: F, t1972: F, t5319: F, t14347: F, t14350: F, t14353: F, t14356: F, t14359: F, t16228: F, t16237: F, t16239: F, t16242: F, t16243: F, t16244: F, t16248: F) -> (F, F, F) {
    let t16249 = t1447 * t6114;
    let t16250 = 4.0 / 45.0 * t16249;
    let t16252 = 2.0 / 15.0 * t1972 * t5319;
    let t16253 = t16228 + 0.04472697096444135 * t14347 + 0.06709045644666203 * t14350 + 0.21642082724729686 * t14353 + 0.8656833089891874 * t14356 + 0.6492624817418906 * t14359 + t16237 + t16239 + t16242 + t16243 + t16244 - t16248 + t16250 + t16252;
    (t16250, t16252, t16253)
}
