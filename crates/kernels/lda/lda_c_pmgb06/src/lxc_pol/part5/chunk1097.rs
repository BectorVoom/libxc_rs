//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1097/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1097<F: Float>(t15947: F, t1924: F, t493: F, t1925: F, t6134: F, t432: F, t7863: F, t161: F, t489: F, t7725: F, t16583: F, t531: F, t7628: F) -> (F, F, F, F, F, F) {
    let t20197 = t493 * t15947 * t1924 / F::cast_from(15.0_f64);
    let t20199 = t6134 * t1925 / F::cast_from(15.0_f64);
    let t20201 = t432 * t7863 / F::cast_from(10.0_f64);
    let t20203 = t161 * t489 * t7725;
    let t20204 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t20203;
    let t20205 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t16583;
    let t20207 = t7628 * t531 / F::cast_from(30.0_f64);
    (t20197, t20199, t20201, t20204, t20205, t20207)
}
