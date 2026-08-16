//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1030/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1030(t12239: f64, t3043: f64, t831: f64, t3461: f64, t3450: f64, t132: f64, t435: f64, t4965: f64, t432: f64, t5120: f64, t1592: f64, t1872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12240 = t12239 / 45.0_f64;
    let t12241 = t831 * t3043;
    let t12242 = 2.0_f64 / 15.0_f64 * t12241;
    let t12244 = t831 * t3461 / 5.0_f64;
    let t12245 = t831 * t3450;
    let t12246 = t12245 / 45.0_f64;
    let t12248 = t132 * t435 * t4965;
    let t12249 = t12248 / 15.0_f64;
    let t12251 = t432 * t5120 / 5.0_f64;
    let t12252 = t1872 * t1592;
    (t12240, t12242, t12244, t12246, t12249, t12251, t12252)
}
