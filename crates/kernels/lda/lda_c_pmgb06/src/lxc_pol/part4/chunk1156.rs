//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1156/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1156<F: Float>(t1080: F, t1414: F, t2599: F, t2871: F, t493: F, t1423: F, t6524: F, t1069: F, t1531: F, t2604: F, t2864: F, t439: F) -> (F, F, F) {
    let t15215 = F::new(4.0) / F::new(45.0) * t493 * t2871 * t2599 * t1414 * t1080;
    let t15216 = t1423 * t6524;
    let t15217 = F::new(8.0) / F::new(135.0) * t15216;
    let t15222 = F::new(4.0) / F::new(45.0) * t439 * t2864 * t2604 * t1531 * t1069;
    (t15215, t15217, t15222)
}
