//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1239/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1239(t12: f64, t2648: f64, t3031: f64, t1594: f64, t1966: f64, t439: f64, t8193: f64, t1083: f64, t12294: f64, t2389: f64, t247: f64, t337: f64, t395: f64, t5974: f64, t6678: f64, t764: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t16317 = t3031 * t2648;
    let t16321 = t439 * t1966 * t16317 * t1594 / 5.0_f64;
    let t16322 = 16.0_f64 * t8193;
    let t16332 = piecewise3(t13, 0.0_f64, 2.0_f64 * t1083 * t2389 + 24.0_f64 * t247 * t6678 + 4.0_f64 * t337 * t5974 - 8.0_f64 * t395 * t764 - t12294 + t16322);
    (t16321, t16322, t16332)
}
