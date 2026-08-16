//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 855/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk855(t219: f64, t1024: f64, t633: f64, t8479: f64, t3952: f64, t654: f64, t957: f64, t682: f64, t696: f64, t978: f64, t1066: f64, t1108: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8499 = 1.0_f64 / t219;
    let t8519 = 6.0_f64 * t1024 * t8479 * t633;
    let t8520 = t3952 * t654;
    let t8522 = t957 * t957;
    let t8526 = 3.5089341735807875_f64 * t696 * t978 * t8522 * t682;
    let t8527 = t1108 * t1066;
    (t8499, t8519, t8520, t8522, t8526, t8527)
}
