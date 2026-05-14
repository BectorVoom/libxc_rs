//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1257/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1257<F: Float>(t2448: F, t384: F, t1147: F, t123: F, t2407: F, t317: F, t113: F, t2414: F, t247: F, t301: F, t10548: F, t73: F, t26: F, t2732: F, t329: F, t1156: F, t2422: F) -> (F, F, F, F, F, F, F) {
    let t18903 = t384 * t2448;
    let t18911 = t123 * t1147 * t2407 * t317;
    let t18915 = t247 * t2414 * t113 * t301;
    let t18926 = t10548 * t73;
    let t18939 = t26 * t2732;
    let t18940 = t329 * t18939;
    let t18954 = t247 * t2407;
    let t18969 = t123 * t1156 * t2422;
    (t18903, t18911, t18915, t18926, t18940, t18954, t18969)
}
