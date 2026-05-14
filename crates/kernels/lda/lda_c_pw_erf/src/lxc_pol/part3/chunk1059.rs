//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1059/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1059<F: Float>(t11308: F, t11310: F, t11311: F, t11312: F, t11314: F, t11316: F, t11318: F, t11320: F, t11323: F, t11324: F, t11328: F, t11331: F, t11333: F, t8168: F, t8177: F, t8188: F) -> (F,) {
    let t14414 = t11308 - t11310 - t8168 - t8177 - t11311 - t11312 - t11314 - t11316 + t11318 + t11320 + t11323 - t8188 - t11324 - t11328 - t11331 - t11333;
    (t14414,)
}
