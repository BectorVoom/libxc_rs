//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1249/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1249<F: Float>(t9596: F, t9598: F, t9601: F, t490: F, t6688: F, t1504: F, t2563: F, t1366: F, t7193: F, t5102: F, t831: F, t161: F, t166: F, t2623: F, t2885: F) -> (F, F, F, F, F, F, F, F) {
    let t16439 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t9596;
    let t16440 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t9598;
    let t16441 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t9601;
    let t16442 = t6688 * t490;
    let t16443 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16442;
    let t16444 = t2563 * t1504;
    let t16445 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16444;
    let t16446 = t7193 * t1366;
    let t16448 = t831 * t5102;
    let t16449 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16448;
    let t16453 = t161 * t166 * t2885 * t2623 / F::cast_from(30.0_f64);
    (t16439, t16440, t16441, t16443, t16445, t16446, t16449, t16453)
}
