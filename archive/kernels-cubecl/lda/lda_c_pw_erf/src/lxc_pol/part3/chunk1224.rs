//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1224/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1224<F: Float>(t11383: F, t11388: F, t11390: F, t11392: F, t11402: F, t11404: F, t11406: F, t11462: F, t11464: F, t8373: F, t8382: F, t8386: F, t8389: F, t8393: F, t8397: F, t8400: F) -> F {
    let t14421 = -t8373 - t11383 - t8382 + t8386 - t11388 + t11390 - t11392 - t8389 - t8393 + t8397 - t8400 + t11402 + t11404 - t11406 + t11462 + t11464;
    t14421
}
