//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1241/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1241<F: Float>(t1318: F, t3854: F, t6404: F, t12976: F, t519: F, t6418: F, t15807: F, t5250: F, t13440: F, t6422: F, t2146: F, t4791: F, t11989: F, t6243: F, t4753: F, t6244: F) -> (F, F, F, F, F, F, F) {
    let t18435 = t1318 * t3854 * t6404;
    let t18436 = 64.0 / 135.0 * t18435;
    let t18438 = t519 * t12976 * t6418;
    let t18439 = 128.0 / 243.0 * t18438;
    let t18442 = 128.0 / 27.0 * t519 * t5250 * t15807;
    let t18444 = t519 * t13440 * t6422;
    let t18445 = 80.0 / 81.0 * t18444;
    let t18446 = t2146 * t4791;
    let t18447 = 64.0 / 135.0 * t18446;
    let t18449 = t1318 * t11989 * t6243;
    let t18450 = 64.0 / 45.0 * t18449;
    let t18452 = 32.0 / 15.0 * t4753 * t6244;
    (t18436, t18439, t18442, t18445, t18447, t18450, t18452)
}
