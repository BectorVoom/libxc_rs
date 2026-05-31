//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 816/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk816<F: Float>(t574: F, t7478: F, t571: F, t2171: F, t2566: F, t3722: F, t7354: F, t1459: F, t519: F, t3714: F, t7365: F, t1485: F) -> (F, F, F, F, F, F, F, F) {
    let t7479 = t574 * t7478;
    let t7481 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t571 * t7479;
    let t7483 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2171 * t2566;
    let t7484 = t3722 * t7354;
    let t7485 = t1459 * t7484;
    let t7487 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t519 * t7485;
    let t7488 = t3714 * t7365;
    let t7489 = t1485 * t7488;
    (t7479, t7481, t7483, t7484, t7485, t7487, t7488, t7489)
}
