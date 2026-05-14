//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1032/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1032<F: Float>(t1972: F, t4589: F, t2002: F, t5203: F, t5198: F, t432: F, t6675: F, t1180: F, t139: F, t30: F) -> (F, F, F, F, F) {
    let t15310 = 2.0 / 27.0 * t1972 * t4589;
    let t15312 = 4.0 / 15.0 * t2002 * t5203;
    let t15314 = 4.0 / 15.0 * t2002 * t5198;
    let t15316 = t432 * t6675 / 15.0;
    let t15323 = t30 * t1180 * t139;
    (t15310, t15312, t15314, t15316, t15323)
}
