//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1120/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1120<F: Float>(t131: F, t155: F, t16769: F, t44: F, t460: F, t6705: F, t1592: F, t6225: F, t1966: F, t439: F, t477: F, t9828: F, t12982: F, t13008: F, t4937: F, t831: F) -> (F, F, F, F, F, F, F) {
    let t16773 = t16769 * t44 * t131 * t155 / 30.0;
    let t16775 = t6705 * t460 / 15.0;
    let t16776 = t1592 * t6225;
    let t16780 = 2.0 / 15.0 * t439 * t1966 * t16776 * t477;
    let t16781 = 4.0 / 405.0 * t9828;
    let t16782 = 8.0 / 81.0 * t12982;
    let t16783 = 16.0 / 135.0 * t13008;
    let t16785 = t831 * t4937 / 15.0;
    (t16773, t16775, t16780, t16781, t16782, t16783, t16785)
}
