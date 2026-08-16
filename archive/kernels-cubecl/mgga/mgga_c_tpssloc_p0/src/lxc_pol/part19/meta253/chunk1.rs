//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 998/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk998<F: Float>(t11588: F, t1184: F, t3451: F, t3447: F, t3448: F, t3475: F, t11549: F, t11556: F, t11558: F, t11561: F, t11563: F, t11566: F, t11572: F, t11576: F, t11580: F, t11585: F, t1174: F) -> (F, F, F) {
    let t11589 = t11588 * t1184;
    let t11590 = t11589 * t3451;
    let t11591 = t3447 * t11590;
    let t11593 = t3448 * t3475;
    let t11594 = t11593 * t3451;
    let t11597 = -F::cast_from(0.86419753086419753084e-3_f64) * t1174 * t11549 + t11556 + F::cast_from(0.55555555555555555554e-3_f64) * t11558 - F::cast_from(0.83333333333333333331e-3_f64) * t11561 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t11563 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t11566 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t11572 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t11576 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t11580 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t11585 + F::cast_from(0.55555555555555555554e-3_f64) * t11591 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t11594;
    (t11589, t11593, t11597)
}
