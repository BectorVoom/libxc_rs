//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1082/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1082<F: Float>(t11382: F, t11388: F, t11404: F, t20070: F, t20074: F, t20075: F, t20077: F, t20078: F, t20079: F, t8373: F, t8382: F, t8386: F, t8389: F, t8393: F, t8397: F, t8400: F) -> F {
    let t20195 = t20070 - t20074 - t20075 - t11382 - t8373 - t8382 + t8386 - t11388 - t8389 - t8393 + t8397 - t8400 + t20077 - t11404 - t20078 + t20079;
    t20195
}
