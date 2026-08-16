//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 736/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk736(t5: f64, t2869: f64, t2881: f64, t1730: f64, t871: f64, t1074: f64, t760: f64, t1: f64, t332: f64, t395: f64, t1881: f64, t247: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t4738 = 4.0_f64 / 135.0_f64 * t2869;
    let t4739 = 2.0_f64 / 45.0_f64 * t2881;
    let t4740 = t871 * t1730;
    let t4742 = t1074 * t760;
    let t4744 = t332 * t1;
    let t4745 = t4744 * t395;
    let t4752 = piecewise3(t6, 0.0_f64, -12.0_f64 * t1881 * t247 + 4.0_f64 * t5 * t395 + 2.0_f64 * t4742 + 8.0_f64 * t4745);
    (t4738, t4739, t4740, t4745, t4752)
}
