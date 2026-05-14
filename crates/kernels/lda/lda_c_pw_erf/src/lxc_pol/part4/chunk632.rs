//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 632/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk632<F: Float>(t560: F, t925: F, t1361: F, t325: F, t1353: F, t1484: F, t56: F) -> (F, F, F, F) {
    let t3627 = t925 * t560;
    let t3629 = t325 * t1361;
    let t3631 = t325 * t1353;
    let t3633 = t56 * t1484;
    (t3627, t3629, t3631, t3633)
}
