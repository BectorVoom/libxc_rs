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
    let t18503 = F::new(3.8973666666666666) * t11228;
    let t18507 = F::new(17.53815) * t38 * t2715 * t1234;
    let t18518 = F::new(70.1526) * t38 * t2712 * t1234;
    let t18542 = piecewise3::<f64>(t6, F::new(0.0), -F::new(56.0) / F::new(81.0) * t8119 * t2377 * t1069 + F::new(64.0) / F::new(27.0) * t4363 * t4745 + F::new(8.0) / F::new(27.0) * t5953 * t1074 - F::new(16.0) / F::new(9.0) * t1212 * t79 * t1100 - F::new(8.0) / F::new(9.0) * t2192 * t395 + F::new(8.0) / F::new(3.0) * t2192 * t2799 + F::new(8.0) / F::new(27.0) * t3537 * t2381 * t1069 - F::new(4.0) / F::new(9.0) * t1212 * t5961 * t332 - F::new(2.0) / F::new(9.0) * t5958 * t1074 + t11259);
    (t18503, t18507, t18518, t18542)
}
