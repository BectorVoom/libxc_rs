//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1124/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1124<F: Float>(t16877: F, t6731: F, t831: F, t16880: F, t16884: F, t13177: F, t16920: F, t16922: F, t16925: F, t16927: F, t16936: F, t16962: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20501 = t16877 / F::cast_from(15.0_f64);
    let t20503 = t831 * t6731 / F::cast_from(5.0_f64);
    let t20504 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16880;
    let t20505 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t16884;
    let t20506 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t13177;
    let t20507 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16920;
    let t20508 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16922;
    let t20509 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16925;
    let t20510 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16927;
    let t20511 = t16936 / F::cast_from(15.0_f64);
    let t20512 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16962;
    (t20501, t20503, t20504, t20505, t20506, t20507, t20508, t20509, t20510, t20511, t20512)
}
