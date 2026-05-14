//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1229/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1229<F: Float>(t3977: F, t6723: F, t13122: F, t4506: F, t13797: F, t14004: F, t14041: F, t14044: F, t1326: F, t15840: F, t519: F, t15844: F, t1991: F, t1446: F, t6443: F, t18213: F, t18214: F, t18215: F, t18216: F, t18220: F, t18223: F, t18227: F, t18231: F, t18234: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18235 = t6723 * t3977;
    let t18238 = 64.0 / 45.0 * t4506 * t13122 * t18235;
    let t18241 = 32.0 / 27.0 * t4506 * t13797 * t18235;
    let t18242 = 16.0 / 135.0 * t14004;
    let t18243 = 64.0 / 135.0 * t14041;
    let t18244 = 32.0 / 405.0 * t14044;
    let t18247 = 8.0 / 15.0 * t519 * t1326 * t15840;
    let t18250 = 16.0 / 3.0 * t519 * t1991 * t15844;
    let t18252 = 64.0 / 45.0 * t1446 * t6443;
    let t18253 = t18213 - t18214 - t18215 + t18216 - t18220 + t18223 + t18227 + t18231 - t18234 - t18238 + t18241 + t18242 - t18243 - t18244 + t18247 + t18250 - t18252;
    (t18238, t18241, t18242, t18243, t18244, t18247, t18250, t18252, t18253)
}
