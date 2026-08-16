//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1163/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1163<F: Float>(t13949: F, t20950: F, t20953: F, t20955: F, t20958: F, t20961: F, t20964: F, t20967: F, t20969: F, t20971: F, t20973: F, t20977: F) -> F {
    let t20978 = -t20950 - t20953 - t20955 + t13949 + t20958 - t20961 + t20964 - t20967 - t20969 - t20971 + t20973 + t20977;
    t20978
}
