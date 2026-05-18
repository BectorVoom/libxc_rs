//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 581/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk581<F: Float>(t5: F, t153: F, t3120: F, t137: F, t132: F, t1542: F, t432: F, t1074: F, t332: F, t3115: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t3121 = t3120 * t153;
    let t3122 = t137 * t3121;
    let t3124 = t132 * t3122 / F::new(30.0);
    let t3126 = t432 * t1542 / F::new(10.0);
    let t3127 = t332 * t1074;
    let t3132 = piecewise3::<f64>(t6, F::new(0.0), F::new(2.0) * t5 * t3115 + F::new(6.0) * t3127);
    let t3133 = t3132 * t44;
    let t3134 = t3133 * t131;
    (t3121, t3122, t3124, t3126, t3127, t3133, t3134)
}
