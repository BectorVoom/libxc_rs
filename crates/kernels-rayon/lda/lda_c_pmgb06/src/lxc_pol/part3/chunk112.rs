//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 112/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk112(t5: f64, t12: f64, t7: f64, t9: f64, t14: f64, t139: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t249 = t7 * zeta_threshold;
    let t250 = t9 * t5;
    let t251 = piecewise3(t6, t249, t250);
    let t252 = t14 * t12;
    let t253 = piecewise3(t13, t249, t252);
    let t254 = t251 + t253 - 2.0_f64;
    let t257 = 1.0_f64 / (2.0_f64 * t139 - 2.0_f64);
    (t250, t252, t254, t257)
}
