//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 206/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk206<F: Float>(t548: F, t551: F, t174: F, t205: F, t499: F, t213: F, t56: F) -> (F, F, F, F) {
    let t553 = 4.0 / 15.0 * t548 * t551;
    let t555 = t174 * t499 * t205;
    let t556 = 0.0018891666666666666 * t555;
    let t557 = t56 * t213;
    (t553, t555, t556, t557)
}
