//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1058/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1058<F: Float>(t5: F, t12: F, t10: F, t1072: F, t1212: F, t1941: F, t19870: F, t21326: F, t332: F, t4687: F, t594: F, t5961: F, t6329: F, t6698: F, t7284: F, t7290: F, t761: F, t1219: F, t15: F, t19395: F, t1949: F, t21345: F, t337: F, t4700: F, t5974: F, t598: F, t6341: F, t6681: F, t7295: F, t7300: F, t765: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t21873 = piecewise3(t6, 0.0, -80.0 / 81.0 * t1212 * t7284 * t332 + 160.0 / 9.0 * t6329 * t1072 + 80.0 / 9.0 * t761 * t6698 + 80.0 / 3.0 * t4687 * t21326 + 40.0 / 3.0 * t1941 * t5961 + 40.0 / 9.0 * t10 * t7290 * t332 + 8.0 / 3.0 * t594 * t19870);
    let t21891 = piecewise3(t13, 0.0, -80.0 / 81.0 * t1219 * t7295 * t337 - 160.0 / 9.0 * t6341 * t1072 + 80.0 / 9.0 * t765 * t6681 - 80.0 / 3.0 * t4700 * t21345 + 40.0 / 3.0 * t1949 * t5974 + 40.0 / 9.0 * t15 * t7300 * t337 + 8.0 / 3.0 * t598 * t19395);
    (t21873, t21891)
}
