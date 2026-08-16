//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1136/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1136(t12: f64, t1079: f64, t1080: f64, t1083: f64, t1100: f64, t11058: f64, t2133: f64, t2386: f64, t2389: f64, t2799: f64, t337: f64, t3922: f64, t395: f64, t4500: f64, t5423: f64, t5974: f64, t6054: f64, t6059: f64, t79: f64, t8499: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t14933 = piecewise3(t13, 0.0_f64, 40.0_f64 / 81.0_f64 * t8499 * t2386 * t1080 + 64.0_f64 / 27.0_f64 * t4500 * t5423 - 8.0_f64 / 27.0_f64 * t6054 * t1083 + 32.0_f64 / 9.0_f64 * t1079 * t79 * t1100 - 16.0_f64 / 9.0_f64 * t2133 * t395 + 16.0_f64 / 3.0_f64 * t2133 * t2799 - 8.0_f64 / 27.0_f64 * t3922 * t2389 * t1080 + 8.0_f64 / 9.0_f64 * t1079 * t5974 * t337 + 4.0_f64 / 9.0_f64 * t6059 * t1083 - t11058);
    t14933
}
