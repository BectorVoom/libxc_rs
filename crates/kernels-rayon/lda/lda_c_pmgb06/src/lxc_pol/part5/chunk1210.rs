//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1210/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1210(t5: f64, t11796: f64, t11799: f64, t19263: f64, t19265: f64, t19268: f64, t19271: f64, t19274: f64, t19276: f64, t19278: f64, t19280: f64, t19282: f64, t10: f64, t1072: f64, t1212: f64, t1941: f64, t19870: f64, t21326: f64, t332: f64, t4687: f64, t594: f64, t5961: f64, t6329: f64, t6698: f64, t7284: f64, t7290: f64, t761: f64, zeta_threshold: f64) -> (f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t21855 = t19263 + t19265 + t19268 + t19271 - t19274 + t19276 + t19278 + t19280 + 0.299209_f64 * t11796 + t11799 + t19282;
    let t21873 = piecewise3(t6, 0.0_f64, -80.0_f64 / 81.0_f64 * t1212 * t7284 * t332 + 160.0_f64 / 9.0_f64 * t6329 * t1072 + 80.0_f64 / 9.0_f64 * t761 * t6698 + 80.0_f64 / 3.0_f64 * t4687 * t21326 + 40.0_f64 / 3.0_f64 * t1941 * t5961 + 40.0_f64 / 9.0_f64 * t10 * t7290 * t332 + 8.0_f64 / 3.0_f64 * t594 * t19870);
    (t21855, t21873)
}
