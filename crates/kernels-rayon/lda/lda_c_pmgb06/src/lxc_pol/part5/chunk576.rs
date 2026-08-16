//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 576/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk576(t3725: f64, t696: f64, t683: f64, t957: f64, t978: f64, t1179: f64, t282: f64, t55: f64, t691: f64, t674: f64, t962: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3727 = 51.94757731704439_f64 * t696 * t3725;
    let t3729 = t978 * t957 * t683;
    let t3731 = 3.5089341735807875_f64 * t696 * t3729;
    let t3734 = t55 * t1179 * t282;
    let t3736 = 0.0005696894717424259_f64 * t691 * t3734;
    let t3738 = 1.0_f64 / t962 / t674;
    (t3727, t3729, t3731, t3734, t3736, t3738)
}
