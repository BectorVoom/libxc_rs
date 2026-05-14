//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1208/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1208<F: Float>(t2389: F, t3727: F, t3794: F, t6292: F, t6230: F, t12695: F, t1325: F, t6454: F, t1639: F, t20: F, t6887: F, t13470: F, t13478: F, t13480: F, t13493: F, t13495: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17900 = 8.0 / 45.0 * t3727 * t2389;
    let t17901 = t3794 * t6292;
    let t17902 = 64.0 / 135.0 * t17901;
    let t17904 = 32.0 / 45.0 * t3794 * t6230;
    let t17906 = t1325 * t12695 * t6454;
    let t17907 = 32.0 / 27.0 * t17906;
    let t17909 = t6887 * t20 * t1639;
    let t17911 = 32.0 / 45.0 * t13470;
    let t17912 = 32.0 / 405.0 * t13478;
    let t17913 = 32.0 / 45.0 * t13480;
    let t17914 = 16.0 / 15.0 * t13493;
    let t17915 = 32.0 / 135.0 * t13495;
    (t17900, t17902, t17904, t17907, t17909, t17911, t17912, t17913, t17914, t17915)
}
