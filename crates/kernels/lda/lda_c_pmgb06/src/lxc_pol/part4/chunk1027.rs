//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1027/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1027<F: Float>(t439: F, t5344: F, t5482: F, t1447: F, t6783: F, t1080: F, t1380: F, t1414: F, t2623: F, t493: F, t1925: F, t5194: F, t1972: F, t5359: F, t1423: F, t6788: F) -> (F, F, F, F, F, F) {
    let t15236 = 4.0 / 45.0 * t439 * t5482 * t5344;
    let t15237 = t1447 * t6783;
    let t15238 = 4.0 / 135.0 * t15237;
    let t15243 = 2.0 / 45.0 * t493 * t1380 * t2623 * t1414 * t1080;
    let t15244 = t5194 * t1925;
    let t15245 = 8.0 / 135.0 * t15244;
    let t15247 = 4.0 / 45.0 * t1972 * t5359;
    let t15248 = t1423 * t6788;
    (t15236, t15238, t15243, t15245, t15247, t15248)
}
