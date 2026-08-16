//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1044/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1044(t5: f64, t12353: f64, t12410: f64, t132: f64, t137: f64, t153: f64, t3122: f64, t802: f64, t1881: f64, t642: f64, t1: f64, t1074: f64, t247: f64, t3115: f64, t395: f64, t4367: f64, t4744: f64, t760: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t12415 = t132 * t137 * (t12353 + t12410) * t153 / 30.0_f64;
    let t12417 = t802 * t3122 / 30.0_f64;
    let t12429 = 48.0_f64 * t1881 * t642;
    let t12431 = piecewise3(t6, 0.0_f64, 12.0_f64 * t1 * t1074 * t395 - 36.0_f64 * t247 * t4744 - 24.0_f64 * t247 * t5 + 2.0_f64 * t3115 * t760 + t12429 + 12.0_f64 * t4367);
    (t12415, t12417, t12431)
}
