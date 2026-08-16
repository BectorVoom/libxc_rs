//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 498/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk498(t5: f64, t12: f64, t10: f64, t2377: f64, t2381: f64, t594: f64, t15: f64, t2386: f64, t2389: f64, t598: f64, t44: f64, t1929: f64, t1931: f64, t1934: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t2510 = piecewise3(t6, 0.0_f64, 40.0_f64 / 9.0_f64 * t10 * t2377 + 8.0_f64 / 3.0_f64 * t594 * t2381);
    let t2516 = piecewise3(t13, 0.0_f64, 40.0_f64 / 9.0_f64 * t15 * t2386 + 8.0_f64 / 3.0_f64 * t598 * t2389);
    let t2519 = (t2510 / 2.0_f64 + t2516 / 2.0_f64) * t44;
    let t2522 = 2.0_f64 / 45.0_f64 * t1929;
    let t2523 = 2.0_f64 / 45.0_f64 * t1931;
    let t2524 = 2.0_f64 / 45.0_f64 * t1934;
    (t2519, t2522, t2523, t2524)
}
