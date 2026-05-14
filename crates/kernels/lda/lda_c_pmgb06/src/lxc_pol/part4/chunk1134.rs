//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1134/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1134<F: Float>(t2466: F, t3198: F, t13204: F, t13206: F, t1444: F, t6541: F, t6545: F, t2465: F, t3194: F, t493: F, t1450: F, t6544: F, t1969: F, t5220: F, t1981: F, t1982: F, t5312: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16979 = t3198 * t2466 / 45.0;
    let t16980 = 4.0 / 135.0 * t13204;
    let t16981 = 8.0 / 135.0 * t13206;
    let t16983 = 2.0 / 45.0 * t1444 * t6541;
    let t16985 = 2.0 / 45.0 * t1444 * t6545;
    let t16988 = t493 * t3194 * t2465 / 45.0;
    let t16991 = 2.0 / 45.0 * t493 * t1450 * t6544;
    let t16992 = t5220 * t1969;
    let t16993 = 8.0 / 45.0 * t16992;
    let t16996 = 8.0 / 45.0 * t1981 * t5312 * t1982;
    (t16979, t16980, t16981, t16983, t16985, t16988, t16991, t16993, t16996)
}
