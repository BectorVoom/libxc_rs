//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1275/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1275<F: Float>(t131: F, t155: F, t16769: F, t44: F, t460: F, t6705: F, t1592: F, t6225: F, t1966: F, t439: F, t477: F, t9828: F) -> (F, F, F, F) {
    let t16773 = t16769 * t44 * t131 * t155 / F::cast_from(30.0_f64);
    let t16775 = t6705 * t460 / F::cast_from(15.0_f64);
    let t16776 = t1592 * t6225;
    let t16780 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t1966 * t16776 * t477;
    let t16781 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t9828;
    (t16773, t16775, t16780, t16781)
}
