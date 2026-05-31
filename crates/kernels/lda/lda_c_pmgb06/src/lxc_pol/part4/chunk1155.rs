//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1155/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1155<F: Float>(t15208: F, t15173: F, t15176: F, t15179: F, t15181: F, t15183: F, t15185: F, t15188: F, t15190: F, t15195: F, t15197: F, t15199: F, t15203: F, t15207: F) -> (F, F) {
    let t15209 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t15208;
    let t15210 = t15173 + t15176 + t15179 - t15181 - t15183 - t15185 + t15188 - t15190 + t15195 - t15197 + t15199 + t15203 + t15207 + t15209;
    (t15209, t15210)
}
