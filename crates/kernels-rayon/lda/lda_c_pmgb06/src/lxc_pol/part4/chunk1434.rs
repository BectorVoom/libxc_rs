//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1434/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1434(t12: f64, t1080: f64, t1083: f64, t1100: f64, t12960: f64, t15: f64, t1949: f64, t2441: f64, t2443: f64, t2799: f64, t337: f64, t395: f64, t5423: f64, t5974: f64, t6341: f64, t6346: f64, t765: f64, t79: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t18377 = piecewise3(t13, 0.0_f64, -80.0_f64 / 81.0_f64 * t2441 * t1080 - 640.0_f64 / 27.0_f64 * t765 * t5423 + 80.0_f64 / 27.0_f64 * t6341 * t1083 + 320.0_f64 / 9.0_f64 * t15 * t79 * t1100 - 160.0_f64 / 9.0_f64 * t1949 * t395 + 160.0_f64 / 3.0_f64 * t1949 * t2799 + 80.0_f64 / 27.0_f64 * t2443 * t1080 + 80.0_f64 / 9.0_f64 * t15 * t5974 * t337 + 40.0_f64 / 9.0_f64 * t6346 * t1083 - t12960);
    t18377
}
