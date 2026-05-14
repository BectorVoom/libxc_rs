//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1228/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1228<F: Float>(t5: F, t10: F, t1069: F, t1074: F, t1100: F, t12939: F, t1941: F, t2435: F, t2437: F, t2799: F, t332: F, t395: F, t4745: F, t5961: F, t6329: F, t6334: F, t761: F, t79: F, zeta_threshold: F) -> (F,) {
    let t6 = t5 <= zeta_threshold;
    let t18355 = piecewise3(t6, 0.0, -80.0 / 81.0 * t2435 * t1069 + 640.0 / 27.0 * t761 * t4745 + 80.0 / 27.0 * t6329 * t1074 + 320.0 / 9.0 * t10 * t79 * t1100 + 160.0 / 9.0 * t1941 * t395 - 160.0 / 3.0 * t1941 * t2799 + 80.0 / 27.0 * t2437 * t1069 + 80.0 / 9.0 * t10 * t5961 * t332 + 40.0 / 9.0 * t6334 * t1074 + t12939);
    (t18355,)
}
