//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1133/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1133<F: Float>(t16964: F, t1447: F, t6545: F, t2470: F, t3226: F, t6282: F, t13196: F, t2002: F, t4609: F, t13199: F, t13201: F, t16952: F, t16954: F, t16956: F, t16959: F, t16961: F, t16963: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16965 = 4.0 / 135.0 * t16964;
    let t16966 = t1447 * t6545;
    let t16967 = 4.0 / 135.0 * t16966;
    let t16968 = t3226 * t2470;
    let t16969 = 4.0 / 81.0 * t16968;
    let t16970 = t1447 * t6282;
    let t16971 = 4.0 / 81.0 * t16970;
    let t16972 = 16.0 / 135.0 * t13196;
    let t16974 = 2.0 / 15.0 * t2002 * t4609;
    let t16975 = 16.0 / 135.0 * t13199;
    let t16976 = 16.0 / 135.0 * t13201;
    let t16977 = -t16952 - t16954 - t16956 - t16959 - t16961 + t16963 + t16965 + t16967 + t16969 + t16971 + t16972 + t16974 - t16975 - t16976;
    (t16965, t16967, t16969, t16971, t16972, t16974, t16975, t16976, t16977)
}
