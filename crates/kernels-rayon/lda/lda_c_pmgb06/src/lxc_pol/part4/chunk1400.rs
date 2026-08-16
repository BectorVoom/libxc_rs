//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1400/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1400(t12224: f64, t12227: f64, t16237: f64, t16239: f64, t16242: f64, t16243: f64, t16244: f64, t16248: f64, t16250: f64, t16252: f64, t16255: f64, t16259: f64, t9478: f64, t9481: f64, t9483: f64) -> f64 {
    let t18218 = t9478 + t9481 + 0.36466666666666664_f64 * t9483 - 8.0_f64 / 27.0_f64 * t12224 - 4.0_f64 / 9.0_f64 * t12227 + t16237 + t16239 + t16242 + t16243 + t16244 - t16248 + t16250 + t16252 + t16255 + t16259;
    t18218
}
