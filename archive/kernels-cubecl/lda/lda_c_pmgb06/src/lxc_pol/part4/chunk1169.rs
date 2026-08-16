//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1169/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1169<F: Float>(t1074: F, t6145: F, t1525: F, t36: F, t1069: F, t2377: F, t9220: F, t3090: F, t6150: F, t9190: F, t9188: F, t350: F, t6186: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15373 = t6145 * t1074;
    let t15375 = t36 * t1525 * t15373;
    let t15378 = t9220 * t2377 * t1069;
    let t15380 = t36 * t3090 * t15378;
    let t15382 = t6150 * t1074;
    let t15384 = t36 * t3090 * t15382;
    let t15387 = t9190 * t2377 * t1069;
    let t15389 = t36 * t9188 * t15387;
    let t15391 = t350 * t6186;
    (t15373, t15375, t15378, t15380, t15382, t15384, t15387, t15389, t15391)
}
