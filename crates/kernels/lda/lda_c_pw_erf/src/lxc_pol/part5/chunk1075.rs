//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1075/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1075<F: Float>(t1325: F, t3787: F, t7576: F, t2120: F, t6220: F, t6209: F, t18138: F, t21577: F, t2504: F, t266: F, t514: F, t548: F, t7837: F, t18158: F, t18163: F, t15582: F, t2193: F) -> (F, F, F, F, F, F, F, F) {
    let t22349 = t1325 * t3787 * t7576;
    let t22350 = 16.0 / 15.0 * t22349;
    let t22351 = t2120 * t6220;
    let t22352 = 8.0 / 15.0 * t22351;
    let t22353 = t6209 * t6220;
    let t22354 = 8.0 / 15.0 * t22353;
    let t22358 = 4.0 / 5.0 * t21577 * t18138 * t266 * t2504;
    let t22360 = t548 * t514 * t7837;
    let t22361 = 8.0 / 45.0 * t22360;
    let t22362 = 16.0 / 15.0 * t18158;
    let t22363 = 8.0 / 15.0 * t18163;
    let t22367 = 4.0 / 5.0 * t15582 * t2193;
    (t22350, t22352, t22354, t22358, t22361, t22362, t22363, t22367)
}
