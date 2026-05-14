//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 489/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk489<F: Float>(t2131: F, t493: F, t514: F, t807: F, t185: F, t812: F) -> (F, F, F, F) {
    let t2133 = 4.0 / 15.0 * t493 * t2131;
    let t2134 = t514 * t807;
    let t2135 = t185 * t2134;
    let t2136 = 4.0 / 45.0 * t2135;
    let t2137 = t514 * t812;
    (t2133, t2134, t2136, t2137)
}
