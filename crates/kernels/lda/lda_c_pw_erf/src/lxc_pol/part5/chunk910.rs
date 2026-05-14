//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 910/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk910<F: Float>(t3863: F, t571: F, t6396: F, t13444: F, t6400: F, t13080: F, t1318: F, t6482: F, t1472: F, t6236: F, t3416: F, t6239: F, t6478: F, t4753: F, t10467: F, t2396: F, t519: F) -> (F, F, F, F, F, F, F, F) {
    let t17684 = t571 * t3863 * t6396;
    let t17687 = t571 * t13444 * t6400;
    let t17690 = t1318 * t13080 * t6482;
    let t17692 = t1472 * t6236;
    let t17694 = t3416 * t6239;
    let t17697 = t1318 * t13080 * t6478;
    let t17699 = t4753 * t6239;
    let t17709 = t519 * t10467 * t2396;
    (t17684, t17687, t17690, t17692, t17694, t17697, t17699, t17709)
}
