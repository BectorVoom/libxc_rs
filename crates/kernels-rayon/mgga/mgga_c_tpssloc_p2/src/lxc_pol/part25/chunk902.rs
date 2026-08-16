//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 902/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk902(t11588: f64, t1184: f64, t3451: f64, t3447: f64, t3448: f64, t3475: f64, t11549: f64, t11556: f64, t11558: f64, t11561: f64, t11563: f64, t11566: f64, t11572: f64, t11576: f64, t11580: f64, t11585: f64, t1174: f64) -> f64 {
    let t11589 = t11588 * t1184;
    let t11590 = t11589 * t3451;
    let t11591 = t3447 * t11590;
    let t11593 = t3448 * t3475;
    let t11594 = t11593 * t3451;
    let t11597 = -0.86419753086419753084e-3_f64 * t1174 * t11549 + t11556 + 0.55555555555555555554e-3_f64 * t11558 - 0.83333333333333333331e-3_f64 * t11561 - 0.16666666666666666666e-2_f64 * t3447 * t11563 + 0.11111111111111111111e-2_f64 * t3447 * t11566 - 0.11111111111111111111e-2_f64 * t3447 * t11572 + 0.83333333333333333331e-3_f64 * t3447 * t11576 + 0.83333333333333333331e-3_f64 * t3447 * t11580 + 0.16666666666666666666e-2_f64 * t3447 * t11585 + 0.55555555555555555554e-3_f64 * t11591 + 0.83333333333333333331e-3_f64 * t3447 * t11594;
    t11597
}
