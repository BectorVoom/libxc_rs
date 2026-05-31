//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1242/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1242<F: Float>(t16354: F, t1919: F, t493: F, t1080: F, t2386: F, t9525: F, t5470: F, t1444: F, t6509: F, t12580: F, t6508: F, t16321: F, t16336: F, t16338: F, t16339: F, t16340: F, t16342: F, t16345: F, t16347: F, t16349: F, t16351: F, t16353: F) -> (F, F, F, F, F, F) {
    let t16357 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t493 * t1919 * t16354;
    let t16359 = t9525 * t2386 * t1080;
    let t16362 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t493 * t5470 * t16359;
    let t16364 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t1444 * t6509;
    let t16367 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t493 * t12580 * t6508;
    let t16368 = -t16321 + t16336 + t16338 - t16339 - t16340 + t16342 + t16345 + t16347 + t16349 + t16351 + t16353 - t16357 - t16362 + t16364 + t16367;
    (t16357, t16359, t16362, t16364, t16367, t16368)
}
