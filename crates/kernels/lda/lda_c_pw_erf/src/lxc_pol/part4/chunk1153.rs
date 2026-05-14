//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1153/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1153<F: Float>(t2824: F, t573: F, t1334: F, t35: F, t571: F, t3794: F, t6999: F, t1446: F, t7004: F, t9593: F, t1982: F, t2134: F, t9596: F, t12527: F, t12529: F, t12532: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16979 = t2824 * t573;
    let t16983 = 64.0 / 45.0 * t571 * t16979 * t1334 * t35;
    let t16985 = 8.0 / 15.0 * t3794 * t6999;
    let t16987 = 8.0 / 15.0 * t1446 * t7004;
    let t16988 = 64.0 / 405.0 * t9593;
    let t16989 = t1982 * t2134;
    let t16990 = 16.0 / 45.0 * t16989;
    let t16991 = 8.0 / 135.0 * t9596;
    let t16992 = 16.0 / 45.0 * t12527;
    let t16993 = 32.0 / 45.0 * t12529;
    let t16994 = 32.0 / 45.0 * t12532;
    (t16979, t16983, t16985, t16987, t16988, t16990, t16991, t16992, t16993, t16994)
}
