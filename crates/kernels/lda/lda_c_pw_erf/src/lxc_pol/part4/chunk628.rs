//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 628/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk628<F: Float>(t506: F, t925: F, t1257: F, t325: F, t1247: F, t1458: F, t56: F) -> (F, F, F, F) {
    let t3530 = t925 * t506;
    let t3532 = t325 * t1257;
    let t3534 = t325 * t1247;
    let t3536 = t56 * t1458;
    (t3530, t3532, t3534, t3536)
}
