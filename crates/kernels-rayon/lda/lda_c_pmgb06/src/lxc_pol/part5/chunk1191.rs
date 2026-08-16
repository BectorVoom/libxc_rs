//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1191/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1191(t21375: f64, t69: f64, t21378: f64, t11311: f64, t11318: f64, t21386: f64, t21389: f64, t21394: f64, t21399: f64, t2209: f64, t2247: f64, t2248: f64, t2448: f64, t5980: f64, t769: f64, t8263: f64, t8287: f64, t8295: f64) -> f64 {
    let t21577 = t69 * t21375;
    let t21581 = t69 * t21378;
    let t21583 = 15.518295_f64 * t2247 * t2248 * t2209 * t2448 + 15.518295_f64 * t2247 * t2248 * t769 * t5980 + t21386 + t21389 + t8263 + t21399 + t8287 - t8295 + 6.89702_f64 * t21577 - 1.724255_f64 * t69 * t21394 + 0.5747516666666667_f64 * t21581 - t11311 + t11318;
    t21583
}
