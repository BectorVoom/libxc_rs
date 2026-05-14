//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1009/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1009<F: Float>(t5: F, t5980: F, t73: F, t1068: F, t1069: F, t1074: F, t1100: F, t11032: F, t2125: F, t2377: F, t2381: F, t2799: F, t332: F, t3912: F, t395: F, t4486: F, t4745: F, t5961: F, t6042: F, t6047: F, t79: F, t8485: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t14875 = t73 * t5980;
    let t14909 = piecewise3(t6, 0.0, 40.0 / 81.0 * t8485 * t2377 * t1069 - 64.0 / 27.0 * t4486 * t4745 - 8.0 / 27.0 * t6042 * t1074 + 32.0 / 9.0 * t1068 * t79 * t1100 + 16.0 / 9.0 * t2125 * t395 - 16.0 / 3.0 * t2125 * t2799 - 8.0 / 27.0 * t3912 * t2381 * t1069 + 8.0 / 9.0 * t1068 * t5961 * t332 + 4.0 / 9.0 * t6047 * t1074 + t11032);
    (t14875, t14909)
}
