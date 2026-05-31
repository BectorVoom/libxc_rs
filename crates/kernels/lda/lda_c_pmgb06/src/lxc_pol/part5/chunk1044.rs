//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1044/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1044<F: Float>(t15793: F, t15795: F, t1600: F, t7857: F, t1992: F, t493: F, t529: F, t132: F, t435: F, t7812: F, t11884: F, t9350: F) -> (F, F, F, F, F, F) {
    let t19498 = t15793 / F::cast_from(15.0_f64);
    let t19499 = t15795 / F::cast_from(15.0_f64);
    let t19500 = t1600 * t7857;
    let t19504 = t493 * t1992 * t19500 * t529 / F::cast_from(15.0_f64);
    let t19506 = t132 * t435 * t7812;
    let t19507 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t19506;
    let t19508 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t11884;
    let t19509 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t9350;
    (t19498, t19499, t19504, t19507, t19508, t19509)
}
