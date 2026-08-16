//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1201/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1201(t12: f64, t1072: f64, t1079: f64, t14: f64, t19395: f64, t2133: f64, t21345: f64, t337: f64, t4500: f64, t4503: f64, t5974: f64, t6054: f64, t6681: f64, t7295: f64, t7300: f64, t8499: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t21768 = piecewise3(t13, 0.0_f64, 40.0_f64 / 81.0_f64 * t8499 * t7295 * t337 + 16.0_f64 / 9.0_f64 * t6054 * t1072 - 8.0_f64 / 9.0_f64 * t4500 * t6681 - 8.0_f64 / 3.0_f64 * t4503 * t21345 + 4.0_f64 / 3.0_f64 * t2133 * t5974 + 4.0_f64 / 9.0_f64 * t1079 * t7300 * t337 + 4.0_f64 / 3.0_f64 * t14 * t19395);
    t21768
}
