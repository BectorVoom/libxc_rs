//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1290/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1290<F: Float>(t13037: F, t13039: F, t13042: F, t13044: F, t13046: F, t13049: F, t13052: F, t13055: F, t13057: F, t13059: F, t13064: F, t13067: F, t13069: F) -> F {
    let t15067 = -t13037 - t13039 - t13042 - t13044 - t13046 + t13049 + t13052 - t13055 - t13057 + t13059 - t13064 - t13067 + t13069;
    t15067
}
