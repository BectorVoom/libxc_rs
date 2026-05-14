//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1014/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1014<F: Float>(t13893: F, t1447: F, t5448: F, t2979: F, t493: F, t5358: F, t1080: F, t1380: F, t1414: F, t2088: F, t1894: F, t3220: F, t1898: F, t1902: F, t1423: F, t5287: F) -> (F, F, F, F, F, F, F, F) {
    let t13894 = 8.0 / 45.0 * t13893;
    let t13895 = t1447 * t5448;
    let t13896 = 8.0 / 45.0 * t13895;
    let t13899 = 2.0 / 15.0 * t493 * t2979 * t5358;
    let t13904 = 2.0 / 15.0 * t493 * t1380 * t2088 * t1414 * t1080;
    let t13905 = t3220 * t1894;
    let t13906 = 4.0 / 45.0 * t13905;
    let t13907 = t3220 * t1898;
    let t13908 = 8.0 / 45.0 * t13907;
    let t13909 = t3220 * t1902;
    let t13910 = 4.0 / 27.0 * t13909;
    let t13911 = t1423 * t5287;
    (t13894, t13896, t13899, t13904, t13906, t13908, t13910, t13911)
}
