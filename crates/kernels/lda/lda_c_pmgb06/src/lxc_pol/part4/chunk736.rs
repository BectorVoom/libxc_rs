//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 736/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk736<F: Float>(t5: F, t2869: F, t2881: F, t1730: F, t871: F, t1074: F, t760: F, t1: F, t332: F, t395: F, t1881: F, t247: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t4738 = F::new(4.0) / F::new(135.0) * t2869;
    let t4739 = F::new(2.0) / F::new(45.0) * t2881;
    let t4740 = t871 * t1730;
    let t4742 = t1074 * t760;
    let t4744 = t332 * t1;
    let t4745 = t4744 * t395;
    let t4752 = piecewise3::<F>(t6, F::new(0.0), -F::new(12.0) * t1881 * t247 + F::new(4.0) * t5 * t395 + F::new(2.0) * t4742 + F::new(8.0) * t4745);
    (t4738, t4739, t4740, t4745, t4752)
}
