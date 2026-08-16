//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 972/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk972(t11296: f64, t11297: f64, t1227: f64, t2209: f64, t2247: f64, t2248: f64, t342: f64, t3559: f64, t4394: f64, t769: f64, t8263: f64, t8287: f64, t8295: f64, t8431: f64, t8433: f64, t8435: f64, t8439: f64, t8441: f64) -> f64 {
    let t11511 = 15.518295_f64 * t2247 * t2248 * t4394 * t342 + 15.518295_f64 * t2247 * t2248 * t2209 * t1227 + 5.172765_f64 * t2247 * t2248 * t769 * t3559 + 0.5747516666666667_f64 * t8431 + 6.89702_f64 * t8433 + 6.89702_f64 * t8435 + 5.364348888888889_f64 * t8439 - 2.2990066666666666_f64 * t8441 - t11296 + t8263 + t11297 + t8287 - t8295;
    t11511
}
