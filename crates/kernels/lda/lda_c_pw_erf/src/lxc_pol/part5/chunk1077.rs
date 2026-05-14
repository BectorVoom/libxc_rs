//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1077/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1077<F: Float>(t15579: F, t2168: F, t2143: F, t6988: F, t1472: F, t7489: F, t2562: F, t5334: F, t1446: F, t7485: F, t2566: F, t5327: F, t4738: F, t6689: F, t6693: F, t17637: F, t1996: F, t3965: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22374 = 4.0 / 5.0 * t15579 * t2168;
    let t22375 = t6988 * t2143;
    let t22376 = 16.0 / 45.0 * t22375;
    let t22378 = 8.0 / 9.0 * t1472 * t7489;
    let t22380 = 8.0 / 15.0 * t5334 * t2562;
    let t22382 = 8.0 / 9.0 * t1446 * t7485;
    let t22384 = 8.0 / 15.0 * t5327 * t2566;
    let t22385 = t4738 * t6689;
    let t22386 = 16.0 / 15.0 * t22385;
    let t22388 = 8.0 / 5.0 * t4738 * t6693;
    let t22391 = 8.0 / 15.0 * t3965 * t17637 * t1996;
    (t22374, t22376, t22378, t22380, t22382, t22384, t22386, t22388, t22391)
}
