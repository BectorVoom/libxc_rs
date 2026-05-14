//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1330/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1330<F: Float>(t18194: F, t18195: F, t18196: F, t18198: F, t18200: F, t18202: F, t18204: F, t18208: F, t18209: F, t18210: F, t18213: F, t18214: F, t18215: F, t18216: F, t18220: F, t18223: F, t18227: F) -> (F,) {
    let t19295 = t18194 - t18195 - t18196 - t18198 + t18200 + t18202 - t18204 - t18208 - t18209 + t18210 + t18213 - t18214 - t18215 + t18216 - t18220 + t18223 + t18227;
    (t19295,)
}
