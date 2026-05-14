//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 665/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk665<F: Float>(t2604: F, t443: F, t332: F, t2864: F, t439: F, t1993: F, t2088: F, t1992: F, t493: F, t1444: F, t2462: F, t5312: F, t834: F, t1982: F, t1988: F, t1981: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6522 = t2604 * t443;
    let t6523 = t6522 * t332;
    let t6524 = t2864 * t6523;
    let t6526 = 2.0 / 45.0 * t439 * t6524;
    let t6527 = t1993 * t2088;
    let t6528 = t1992 * t6527;
    let t6530 = 2.0 / 15.0 * t493 * t6528;
    let t6532 = 2.0 / 45.0 * t1444 * t2462;
    let t6533 = t5312 * t834;
    let t6535 = 2.0 / 45.0 * t493 * t6533;
    let t6536 = t1988 * t1982;
    let t6538 = 4.0 / 45.0 * t1981 * t6536;
    (t6522, t6523, t6524, t6526, t6527, t6528, t6530, t6532, t6533, t6535, t6536, t6538)
}
