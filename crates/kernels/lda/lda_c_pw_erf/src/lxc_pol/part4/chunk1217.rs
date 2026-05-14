//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1217/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1217<F: Float>(t13771: F, t13962: F, t15728: F, t10438: F, t18001: F, t18006: F, t18007: F, t18008: F, t18009: F, t18010: F, t18012: F, t18017: F, t18019: F, t18021: F, t18024: F, t18026: F, t18029: F, t18032: F, t18035: F) -> (F, F) {
    let t18038 = 64.0 / 15.0 * t13771 * t13962 * t15728;
    let t18039 = t18001 + t18006 - t18007 + t18008 + t18009 - t10438 - t18010 + t18012 - t18017 - t18019 + t18021 - t18024 + t18026 - t18029 + t18032 - t18035 - t18038;
    (t18038, t18039)
}
