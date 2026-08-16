//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1265/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1265<F: Float>(t12165: F, t12169: F, t12171: F, t12173: F, t12176: F, t12178: F, t12181: F, t12184: F, t12186: F, t12188: F, t12190: F, t12194: F, t12197: F) -> F {
    let t14999 = -t12165 - t12169 - t12171 - t12173 - t12176 - t12178 - t12181 - t12184 - t12186 - t12188 - t12190 - t12194 + t12197;
    t14999
}
