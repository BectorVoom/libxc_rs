//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1150/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1150<F: Float>(t1322: F, t787: F, t117: F, t123: F, t2687: F, t740: F, t1179: F, t2414: F, t419: F, t421: F, t409: F, t6716: F) -> (F, F, F, F) {
    let t15136 = t1322 * t787;
    let t15152 = t123 * t740 * t2687 * t117;
    let t15159 = t1179 * t2414 * t419 * t421;
    let t15163 = t409 * t6716 * t419 * t421;
    (t15136, t15152, t15159, t15163)
}
