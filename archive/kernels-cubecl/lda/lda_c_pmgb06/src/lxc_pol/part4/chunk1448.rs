//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1448/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1448<F: Float>(t5: F, t11228: F, t1234: F, t2715: F, t38: F, t2712: F, t1069: F, t1074: F, t1100: F, t11259: F, t1212: F, t2192: F, t2377: F, t2381: F, t2799: F, t332: F, t3537: F, t395: F, t4363: F, t4745: F, t5953: F, t5958: F, t5961: F, t79: F, t8119: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t18503 = F::cast_from(3.8973666666666666_f64) * t11228;
    let t18507 = F::cast_from(17.53815_f64) * t38 * t2715 * t1234;
    let t18518 = F::cast_from(70.1526_f64) * t38 * t2712 * t1234;
    let t18542 = piecewise3::<F>(t6, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t8119 * t2377 * t1069 + F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t4363 * t4745 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5953 * t1074 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1212 * t79 * t1100 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2192 * t395 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2192 * t2799 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t3537 * t2381 * t1069 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1212 * t5961 * t332 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5958 * t1074 + t11259);
    (t18503, t18507, t18518, t18542)
}
