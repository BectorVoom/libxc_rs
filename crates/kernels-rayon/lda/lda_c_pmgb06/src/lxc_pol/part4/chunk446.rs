//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 446/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk446(t5: f64, t12: f64, t1710: f64, t223: f64, t10: f64, t1069: f64, t1074: f64, t594: f64, t1080: f64, t1083: f64, t15: f64, t598: f64, t44: f64, t208: f64, t81: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t1712 = 2.0_f64 / 135.0_f64 * t223 * t1710;
    let t1718 = piecewise3(t6, 0.0_f64, 40.0_f64 / 9.0_f64 * t10 * t1069 + 8.0_f64 / 3.0_f64 * t594 * t1074);
    let t1724 = piecewise3(t13, 0.0_f64, 40.0_f64 / 9.0_f64 * t15 * t1080 + 8.0_f64 / 3.0_f64 * t598 * t1083);
    let t1727 = (t1718 / 2.0_f64 + t1724 / 2.0_f64) * t44;
    let t1730 = t81 * t208;
    (t1712, t1727, t1730)
}
