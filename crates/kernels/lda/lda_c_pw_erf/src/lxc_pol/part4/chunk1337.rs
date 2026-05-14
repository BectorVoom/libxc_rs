//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1337/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1337<F: Float>(t656: F, t6881: F, t6884: F, t15150: F, t18484: F, t18486: F, t18488: F, t18491: F, t18493: F, t18495: F, t18497: F, t18499: F, t18501: F, t18503: F, t18506: F, t18511: F, t18516: F, t18518: F) -> (F,) {
    let t19318 = t6881 * t656;
    let t19320 = t6884 * t656;
    let t19322 = t15150 + 4.0 / 9.0 * t19318 + 4.0 / 9.0 * t19320 + t18484 + t18486 + t18488 + t18491 + t18493 + t18495 + t18497 + t18499 + t18501 + t18503 - t18506 - t18511 + t18516 + t18518;
    (t19322,)
}
