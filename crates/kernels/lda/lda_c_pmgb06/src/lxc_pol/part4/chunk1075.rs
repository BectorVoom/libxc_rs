//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1075/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1075<F: Float>(t16040: F, t16044: F, t16048: F, t16050: F, t16052: F, t16054: F, t16056: F, t16058: F, t16060: F, t16063: F, t16067: F, t16069: F, t16072: F, t16076: F, t16077: F, t9404: F) -> (F, F) {
    let t16078 = -t16040 - t16044 - t16048 - t16050 + t16052 + t16054 - t16056 - t16058 + t16060 + t16063 + t16067 + t16069 + t16072 + t16076 - t16077;
    let t16083 = 2.0 / 135.0 * t9404;
    (t16078, t16083)
}
