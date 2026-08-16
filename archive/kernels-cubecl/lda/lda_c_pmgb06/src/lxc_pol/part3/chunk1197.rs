//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1197/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1197<F: Float>(t12184: F, t12186: F, t12189: F, t12192: F, t12197: F, t12199: F, t12201: F, t12203: F, t12208: F, t12210: F, t12219: F, t1377: F, t2342: F, t97: F) -> (F, F) {
    let t14345 = -t12184 - t12186 - t12189 + t12192 - t12197 - t12199 - t12201 - t12203 + t12208 + t12210 - t12219;
    let t14347 = t2342 * t97 * t1377;
    (t14345, t14347)
}
