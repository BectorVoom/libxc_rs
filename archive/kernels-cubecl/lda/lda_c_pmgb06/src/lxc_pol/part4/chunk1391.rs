//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1391/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1391<F: Float>(t15925: F, t15927: F, t15930: F, t15934: F, t15939: F, t15942: F, t15944: F, t15946: F, t15950: F, t15953: F, t15955: F, t15957: F, t15959: F, t15962: F, t15965: F) -> F {
    let t18188 = -t15925 - t15927 - t15930 - t15934 - t15939 + t15942 - t15944 - t15946 - t15950 - t15953 - t15955 - t15957 - t15959 + t15962 - t15965;
    t18188
}
