//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 441/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk441(t1680: f64, t209: f64, t1166: f64, t205: f64, t208: f64, t398: f64, t579: f64, t213: f64, t573: f64, t97: f64, t588: f64, t947: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1682 = 2.0_f64 / 27.0_f64 * t209 * t1680;
    let t1683 = t1166 * t205;
    let t1684 = t1683 * t208;
    let t1687 = t398 * t579;
    let t1688 = t1687 * t208;
    let t1689 = t1688 * t213;
    let t1691 = t573 * t97;
    let t1692 = t1691 * t588;
    let t1696 = -0.043111111111111114_f64 * t947 + 0.18777777777777777_f64 * t955;
    (t1682, t1683, t1684, t1687, t1688, t1689, t1691, t1692, t1696)
}
