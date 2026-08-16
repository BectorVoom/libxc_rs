//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 103/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk103(t5: f64, t12: f64, t126: f64, t8: f64, t10: f64, t127: f64, t15: f64, t158: f64, t44: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t216 = t8 * t126;
    let t217 = t10 * t127;
    let t218 = piecewise3(t6, t216, t217);
    let t219 = t15 * t158;
    let t220 = piecewise3(t13, t216, t219);
    let t223 = (t218 / 2.0_f64 + t220 / 2.0_f64) * t44;
    (t217, t219, t223)
}
