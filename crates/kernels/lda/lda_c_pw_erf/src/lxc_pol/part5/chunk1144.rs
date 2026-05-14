//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1144/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1144<F: Float>(t13930: F, t14005: F, t22386: F, t22388: F, t22391: F, t22392: F, t22394: F, t22398: F, t22403: F, t22405: F, t22407: F, t22411: F, t22412: F, t22424: F, t22425: F, t22427: F, t22429: F, t22432: F, t22434: F, t22436: F, t22438: F, t22440: F, t22442: F, t22444: F, t22446: F, t22448: F) -> (F, F) {
    let t23308 = -t22386 - t22388 - t13930 - t22391 - t22392 - t22394 - t22398 - t22403 - t22405 - t22407 + t22411 + t14005 - t22412;
    let t23311 = -t22424 + t22425 - t22427 - t22429 + t22432 + t22434 + t22436 - t22438 - t22440 - t22442 - t22444 + t22446 - t22448;
    (t23308, t23311)
}
