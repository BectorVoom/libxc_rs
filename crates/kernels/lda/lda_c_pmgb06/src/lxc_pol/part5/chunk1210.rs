//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1210/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1210<F: Float>(t5: F, t11796: F, t11799: F, t19263: F, t19265: F, t19268: F, t19271: F, t19274: F, t19276: F, t19278: F, t19280: F, t19282: F, t10: F, t1072: F, t1212: F, t1941: F, t19870: F, t21326: F, t332: F, t4687: F, t594: F, t5961: F, t6329: F, t6698: F, t7284: F, t7290: F, t761: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t21855 = t19263 + t19265 + t19268 + t19271 - t19274 + t19276 + t19278 + t19280 + F::cast_from(0.299209_f64) * t11796 + t11799 + t19282;
    let t21873 = piecewise3::<F>(t6, F::cast_from(0.0_f64), -F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t1212 * t7284 * t332 + F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t6329 * t1072 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t761 * t6698 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t4687 * t21326 + F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t1941 * t5961 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t10 * t7290 * t332 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t594 * t19870);
    (t21855, t21873)
}
