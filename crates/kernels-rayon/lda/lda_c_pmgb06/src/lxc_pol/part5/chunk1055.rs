//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1055/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1055(t12: f64, t6675: f64, t802: f64, t1933: f64, t2563: f64, t1072: f64, t19395: f64, t2389: f64, t337: f64, t5974: f64, t7300: f64, t764: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t19642 = t802 * t6675 / 10.0_f64;
    let t19643 = t2563 * t1933;
    let t19644 = t19643 / 15.0_f64;
    let t19654 = piecewise3(t13, 0.0_f64, -12.0_f64 * t1072 * t2389 + 2.0_f64 * t12 * t19395 + 2.0_f64 * t337 * t7300 + 6.0_f64 * t5974 * t764);
    (t19642, t19644, t19654)
}
