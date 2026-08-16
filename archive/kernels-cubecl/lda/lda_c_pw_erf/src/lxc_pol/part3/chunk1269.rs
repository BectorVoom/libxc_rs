//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1269/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1269<F: Float>(t12316: F, t12320: F, t12327: F, t12331: F, t12337: F, t12339: F, t12341: F, t12345: F, t12348: F, t12351: F, t12355: F, t12357: F, t12361: F) -> F {
    let t15006 = -t12316 + t12320 + t12327 + t12331 + t12337 + t12339 + t12341 + t12345 + t12348 + t12351 + t12355 + t12357 + t12361;
    t15006
}
