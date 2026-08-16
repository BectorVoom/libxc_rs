//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 277/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk277(t5: f64, t12: f64, t760: f64, t9: f64, t14: f64, t764: f64, t257: f64, zeta_threshold: f64) -> f64 {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t892 = piecewise3(t6, 0.0_f64, 4.0_f64 / 3.0_f64 * t9 * t760);
    let t895 = piecewise3(t13, 0.0_f64, 4.0_f64 / 3.0_f64 * t14 * t764);
    let t897 = (t892 + t895) * t257;
    t897
}
