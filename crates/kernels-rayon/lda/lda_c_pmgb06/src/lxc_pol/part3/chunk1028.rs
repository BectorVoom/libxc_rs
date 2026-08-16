//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1028/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1028(t132: f64, t137: f64, t1629: f64, t4815: f64, t1680: f64, t2022: f64, t9457: f64, t9461: f64, t9467: f64, t9470: f64, t9478: f64, t9481: f64, t9483: f64, t9491: f64, t9494: f64) -> (f64, f64) {
    let t12219 = t132 * t137 * t4815 * t1629 / 10.0_f64;
    let t12224 = t2022 * t1680;
    let t12225 = 2.0_f64 / 9.0_f64 * t12224;
    let t12226 = -t12219 + 0.004546314527777778_f64 * t9457 + t9461 + t9467 + t9470 + t9478 + t9481 + 0.547_f64 * t9483 + t9491 / 3.0_f64 + 0.06077777777777778_f64 * t9494 - t12225;
    (t12219, t12226)
}
