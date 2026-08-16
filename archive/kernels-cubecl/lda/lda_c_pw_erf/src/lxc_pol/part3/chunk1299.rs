//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1299/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1299<F: Float>(t13403: F, t13405: F, t13407: F, t13408: F, t13409: F, t13410: F, t13411: F, t13412: F, t13413: F, t13415: F, t13416: F, t13417: F, t13420: F, t13423: F) -> F {
    let t15083 = t13403 - t13405 + t13407 - t13408 - t13409 - t13410 - t13411 + t13412 + t13413 - t13415 + t13416 + t13417 - t13420 - t13423;
    t15083
}
