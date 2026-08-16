//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 695/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk695(t5: f64, t12: f64, t1072: f64, t1941: f64, t332: f64, t594: f64, t5961: f64, t6329: f64, t6334: f64, t2386: f64, t336: f64, t15: f64, t2389: f64, t1949: f64, t337: f64, t5974: f64, t598: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t6340 = piecewise3(t6, 0.0_f64, 80.0_f64 / 27.0_f64 * t6329 * t332 + 160.0_f64 / 9.0_f64 * t1941 * t1072 + 40.0_f64 / 9.0_f64 * t6334 * t332 + 8.0_f64 / 3.0_f64 * t594 * t5961);
    let t6341 = t336 * t2386;
    let t6346 = t15 * t2389;
    let t6352 = piecewise3(t13, 0.0_f64, 80.0_f64 / 27.0_f64 * t6341 * t337 - 160.0_f64 / 9.0_f64 * t1949 * t1072 + 40.0_f64 / 9.0_f64 * t6346 * t337 + 8.0_f64 / 3.0_f64 * t598 * t5974);
    (t6340, t6341, t6352)
}
