//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1335/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1335<F: Float>(t18410: F, t18412: F, t18414: F, t18416: F, t18418: F, t18420: F, t18422: F, t18426: F, t18429: F, t18436: F, t18439: F, t18442: F, t18445: F, t18447: F, t18450: F, t18452: F, t18454: F) -> (F,) {
    let t19306 = -t18410 + t18412 + t18414 + t18416 + t18418 + t18420 + t18422 - t18426 - t18429 - t18436 + t18439 - t18442 - t18445 - t18447 + t18450 + t18452 + t18454;
    (t19306,)
}
