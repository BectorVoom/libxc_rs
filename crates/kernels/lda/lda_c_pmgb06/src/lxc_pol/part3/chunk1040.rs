//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1040/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1040<F: Float>(t12135: F, t12138: F, t12142: F, t12145: F, t12149: F, t12153: F, t12159: F, t12164: F, t12168: F, t12170: F, t12174: F, t12179: F, t12184: F, t12186: F, t12189: F, t12192: F, t12197: F, t12199: F, t12201: F, t12203: F, t12208: F, t12210: F, t12219: F) -> (F, F) {
    let t14343 = t12135 + t12138 + t12142 + t12145 + t12149 + t12153 + t12159 + t12164 - t12168 - t12170 - t12174 - t12179;
    let t14345 = -t12184 - t12186 - t12189 + t12192 - t12197 - t12199 - t12201 - t12203 + t12208 + t12210 - t12219;
    (t14343, t14345)
}
