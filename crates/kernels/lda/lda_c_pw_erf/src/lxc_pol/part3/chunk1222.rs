//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1222/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1222<F: Float>(t11352: F, t11353: F, t11356: F, t11357: F, t11360: F, t11362: F, t11363: F, t11364: F, t11365: F, t8248: F, t8260: F, t8263: F, t8266: F, t8271: F, t8274: F, t8277: F) -> F {
    let t14417 = -t8248 - t11352 + t11353 - t11356 - t11357 + t8260 + t11360 + t11362 + t8263 - t8266 - t11363 + t8271 + t8274 - t8277 - t11364 + t11365;
    t14417
}
