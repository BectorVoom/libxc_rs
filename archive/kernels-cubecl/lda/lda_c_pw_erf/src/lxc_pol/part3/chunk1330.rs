//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1330/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1330<F: Float>(t1746: F, t5949: F, t11266: F, t14439: F, t14440: F, t14441: F, t14442: F, t14444: F, t14446: F, t14448: F, t14450: F, t8527: F, t8533: F, t8536: F, t8539: F, t8542: F, t8716: F, t8733: F, t8737: F, t8740: F) -> F {
    let t15296 = t5949 * t1746;
    let t15297 = F::cast_from(2.0538164420033334_f64) * t15296;
    let t15298 = -t14439 + t14440 + t8527 + t14441 + t8533 - t8536 + t8539 - t8542 - t11266 + t14442 + t14444 - t14446 + t15297 + t14448 - t14450 + t8733 - t8716 - t8737 + t8740;
    t15298
}
