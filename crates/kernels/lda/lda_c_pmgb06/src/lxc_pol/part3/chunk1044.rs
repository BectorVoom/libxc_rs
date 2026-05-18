//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1044/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1044<F: Float>(t5: F, t12353: F, t12410: F, t132: F, t137: F, t153: F, t3122: F, t802: F, t1881: F, t642: F, t1: F, t1074: F, t247: F, t3115: F, t395: F, t4367: F, t4744: F, t760: F, zeta_threshold: F) -> (F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t12415 = t132 * t137 * (t12353 + t12410) * t153 / F::new(30.0);
    let t12417 = t802 * t3122 / F::new(30.0);
    let t12429 = F::new(48.0) * t1881 * t642;
    let t12431 = piecewise3::<f64>(t6, F::new(0.0), F::new(12.0) * t1 * t1074 * t395 - F::new(36.0) * t247 * t4744 - F::new(24.0) * t247 * t5 + F::new(2.0) * t3115 * t760 + t12429 + F::new(12.0) * t4367);
    (t12415, t12417, t12431)
}
