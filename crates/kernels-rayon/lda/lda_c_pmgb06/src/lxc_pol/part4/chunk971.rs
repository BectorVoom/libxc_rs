//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 971/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk971(t2789: f64, t297: f64, t301: f64, t398: f64, t1767: f64, t55: f64, t32: f64, t4238: f64, t107: f64, t4913: f64, t642: f64, t93: f64) -> (f64, f64, f64, f64) {
    let t8163 = t297 * t398 * t2789 * t301;
    let t8165 = t55 * t1767;
    let t8170 = t32 * t4238;
    let t8173 = -70.0_f64 / 81.0_f64 * t93 * t8165 + 0.22252592592592593_f64 * t4913 - 0.07316671043820612_f64 * t642 + 0.015663796296296297_f64 * t107 * t8170;
    (t8163, t8165, t8170, t8173)
}
