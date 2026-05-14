//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 920/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk920<F: Float>(t12118: F, t4502: F, t3675: F, t521: F, t3807: F, t4488: F, t806: F, t3811: F, t4490: F, t4497: F, t12100: F, t12101: F, t12102: F, t12103: F, t12104: F, t12105: F, t12108: F, t12112: F, t12117: F) -> (F, F, F, F, F) {
    let t12119 = t12118 * t4502;
    let t12120 = 16.0 / 27.0 * t12119;
    let t12121 = t521 * t3675;
    let t12125 = 8.0 / 5.0 * t4488 * t12121 * t806 * t3807;
    let t12128 = 16.0 / 15.0 * t4488 * t4490 * t3811;
    let t12129 = t12118 * t4497;
    let t12130 = 32.0 / 45.0 * t12129;
    let t12131 = t12100 + t12101 - t12102 + t12103 + t12104 + t12105 + t12108 - t12112 - t12117 - t12120 - t12125 - t12128 + t12130;
    (t12120, t12125, t12128, t12130, t12131)
}
