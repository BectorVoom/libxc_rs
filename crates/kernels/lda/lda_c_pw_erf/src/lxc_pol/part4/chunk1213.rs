//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1213/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1213<F: Float>(t13829: F, t16843: F, t4506: F, t13035: F, t6740: F, t10015: F, t10419: F, t10422: F, t14048: F, t548: F, t1960: F, t2123: F, t5170: F, t822: F, t515: F, t6788: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17968 = 64.0 / 81.0 * t4506 * t13829 * t16843;
    let t17970 = 32.0 / 45.0 * t13035 * t6740;
    let t17972 = 32.0 / 45.0 * t10015 * t6740;
    let t17975 = 64.0 / 405.0 * t10419;
    let t17976 = 8.0 / 135.0 * t10422;
    let t17978 = 8.0 / 15.0 * t548 * t14048;
    let t17979 = t1960 * t2123;
    let t17980 = 16.0 / 45.0 * t17979;
    let t17981 = t822 * t5170;
    let t17982 = 16.0 / 45.0 * t17981;
    let t17983 = t6788 * t515;
    (t17968, t17970, t17972, t17975, t17976, t17978, t17980, t17982, t17983)
}
