//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1327/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1327<F: Float>(t18010: F, t18012: F, t18017: F, t18019: F, t18021: F, t18024: F, t18026: F, t18029: F, t18032: F, t18035: F, t18038: F, t18046: F, t18048: F, t18050: F, t18052: F, t18111: F, t18112: F) -> (F,) {
    let t19288 = -t18010 + t18012 - t18017 - t18019 + t18021 - t18024 + t18026 - t18029 + t18032 - t18035 - t18038 + t18046 - t18048 - t18050 + t18052 + t18111 - t18112;
    (t19288,)
}
