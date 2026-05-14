//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1100/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1100<F: Float>(t9601: F, t490: F, t6688: F, t1504: F, t2563: F, t1366: F, t7193: F, t5102: F, t831: F, t161: F, t166: F, t2623: F, t2885: F, t16424: F, t16425: F, t16427: F, t16429: F, t16431: F, t16433: F, t16438: F, t16439: F, t16440: F) -> (F, F, F, F, F, F) {
    let t16441 = 2.0 / 135.0 * t9601;
    let t16442 = t6688 * t490;
    let t16443 = 2.0 / 45.0 * t16442;
    let t16444 = t2563 * t1504;
    let t16445 = 2.0 / 45.0 * t16444;
    let t16446 = t7193 * t1366;
    let t16448 = t831 * t5102;
    let t16449 = 4.0 / 45.0 * t16448;
    let t16453 = t161 * t166 * t2885 * t2623 / 30.0;
    let t16454 = -t16424 - t16425 + t16427 - t16429 - t16431 - t16433 + t16438 - t16439 + t16440 + t16441 + t16443 + t16445 + 0.07214027574909895 * t16446 - t16449 - t16453;
    (t16441, t16443, t16445, t16449, t16453, t16454)
}
