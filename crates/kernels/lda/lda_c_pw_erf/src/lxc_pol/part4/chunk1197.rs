//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1197/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1197<F: Float>(t3863: F, t571: F, t6396: F, t13444: F, t6400: F, t13080: F, t1318: F, t6482: F, t1472: F, t6236: F, t3416: F, t6239: F, t6478: F, t4753: F, t12765: F, t1325: F, t2471: F, t494: F, t542: F) -> (F, F, F, F, F, F, F, F) {
    let t17684 = t571 * t3863 * t6396;
    let t17685 = 32.0 / 135.0 * t17684;
    let t17687 = t571 * t13444 * t6400;
    let t17688 = 16.0 / 27.0 * t17687;
    let t17690 = t1318 * t13080 * t6482;
    let t17691 = 32.0 / 27.0 * t17690;
    let t17692 = t1472 * t6236;
    let t17693 = 32.0 / 135.0 * t17692;
    let t17694 = t3416 * t6239;
    let t17695 = 64.0 / 135.0 * t17694;
    let t17697 = t1318 * t13080 * t6478;
    let t17698 = 16.0 / 27.0 * t17697;
    let t17699 = t4753 * t6239;
    let t17700 = 64.0 / 135.0 * t17699;
    let t17705 = 16.0 / 5.0 * t1325 * t12765 * t2471 * t542 * t494;
    (t17685, t17688, t17691, t17693, t17695, t17698, t17700, t17705)
}
