//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 968/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk968<F: Float>(t1592: F, t1966: F, t439: F, t477: F, t5039: F, t9936: F, t161: F, t166: F, t851: F, t9603: F, t1995: F, t3223: F, t13112: F, t13114: F, t13116: F, t13118: F, t13120: F, t13123: F, t13125: F, t13128: F) -> (F, F, F, F, F) {
    let t13133 = t439 * t1966 * t1592 * t5039 * t477 / 5.0;
    let t13134 = t9936 / 45.0;
    let t13138 = t161 * t166 * t9603 * t851 / 30.0;
    let t13139 = t3223 * t1995;
    let t13140 = 2.0 / 45.0 * t13139;
    let t13141 = t13112 + t13114 + t13116 + t13118 + t13120 + t13123 - t13125 - t13128 + t13133 - t13134 - t13138 - t13140;
    (t13133, t13134, t13138, t13140, t13141)
}
