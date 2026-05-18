//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1172/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1172<F: Float>(t1069: F, t2381: F, t3092: F, t3090: F, t36: F, t2579: F, t947: F, t2571: F, t1525: F, t1830: F, t1858: F, t2575: F) -> (F, F, F, F, F, F) {
    let t15411 = t3092 * t2381 * t1069;
    let t15413 = t36 * t3090 * t15411;
    let t15416 = t947 * t2579;
    let t15418 = t947 * t2571;
    let t15421 = t1830 * t1525 * t1858;
    let t15423 = t947 * t2575;
    (t15411, t15413, t15416, t15418, t15421, t15423)
}
