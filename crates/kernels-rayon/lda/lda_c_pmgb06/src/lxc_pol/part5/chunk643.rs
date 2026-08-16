//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 643/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk643(t5378: f64, t588: f64, t1798: f64, t579: f64, t208: f64, t213: f64, t2021: f64, t97: f64, t4876: f64, t1450: f64, t176: f64, t3238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5379 = t5378 * t588;
    let t5385 = t1798 * t579;
    let t5386 = t5385 * t208;
    let t5388 = 2.0_f64 / 3.0_f64 * t5386 * t213;
    let t5391 = t2021 * t97;
    let t5393 = 0.12155555555555556_f64 * t5391 * t588;
    let t5405 = 0.002518888888888889_f64 * t4876;
    let t5447 = t1450 * t176;
    let t5463 = t3238 * t176;
    (t5379, t5385, t5386, t5388, t5391, t5393, t5405, t5447, t5463)
}
