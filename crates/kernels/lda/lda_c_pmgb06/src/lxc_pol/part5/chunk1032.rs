//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1032/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1032<F: Float>(t5: F, t11228: F, t2381: F, t395: F, t1072: F, t1212: F, t19870: F, t2192: F, t330: F, t332: F, t4363: F, t4366: F, t5953: F, t5961: F, t6698: F, t7284: F, t7290: F, t8119: F, zeta_threshold: F) -> (F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t21317 = 5.84605 * t11228;
    let t21326 = t395 * t2381;
    let t21337 = piecewise3(t6, 0.0, -56.0 / 81.0 * t8119 * t7284 * t332 + 16.0 / 9.0 * t5953 * t1072 + 8.0 / 9.0 * t4363 * t6698 - 4.0 / 3.0 * t4366 * t21326 - 2.0 / 3.0 * t2192 * t5961 - 2.0 / 9.0 * t1212 * t7290 * t332 + 2.0 / 3.0 * t330 * t19870);
    (t21317, t21326, t21337)
}
