//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 679/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk679<F: Float>(t3437: F, t548: F, t1529: F, t822: F, t1982: F, t515: F, t1960: F, t568: F, t3380: F, t3385: F, t3388: F, t3391: F, t3975: F, t811: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4464 = 4.0 / 15.0 * t548 * t3437;
    let t4465 = t822 * t1529;
    let t4466 = 4.0 / 135.0 * t4465;
    let t4468 = 8.0 / 45.0 * t1982 * t515;
    let t4470 = 8.0 / 45.0 * t1960 * t568;
    let t4471 = 16.0 / 45.0 * t3380;
    let t4472 = 8.0 / 45.0 * t3385;
    let t4473 = 8.0 / 45.0 * t3388;
    let t4474 = 8.0 / 45.0 * t3391;
    let t4475 = t3975 * t811;
    (t4464, t4465, t4466, t4468, t4470, t4471, t4472, t4473, t4474, t4475)
}
