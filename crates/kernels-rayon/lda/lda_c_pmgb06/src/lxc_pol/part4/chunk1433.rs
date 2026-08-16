//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1433/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1433(t5: f64, t10: f64, t1069: f64, t1074: f64, t1100: f64, t12939: f64, t1941: f64, t2435: f64, t2437: f64, t2799: f64, t332: f64, t395: f64, t4745: f64, t5961: f64, t6329: f64, t6334: f64, t761: f64, t79: f64, zeta_threshold: f64) -> f64 {
    let t6 = t5 <= zeta_threshold;
    let t18355 = piecewise3(t6, 0.0_f64, -80.0_f64 / 81.0_f64 * t2435 * t1069 + 640.0_f64 / 27.0_f64 * t761 * t4745 + 80.0_f64 / 27.0_f64 * t6329 * t1074 + 320.0_f64 / 9.0_f64 * t10 * t79 * t1100 + 160.0_f64 / 9.0_f64 * t1941 * t395 - 160.0_f64 / 3.0_f64 * t1941 * t2799 + 80.0_f64 / 27.0_f64 * t2437 * t1069 + 80.0_f64 / 9.0_f64 * t10 * t5961 * t332 + 40.0_f64 / 9.0_f64 * t6334 * t1074 + t12939);
    t18355
}
