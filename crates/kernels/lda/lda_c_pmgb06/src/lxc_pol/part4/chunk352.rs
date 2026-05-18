//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 352/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk352<F: Float>(t109: F, t342: F, t55: F, t1243: F, t349: F, t947: F) -> (F, F, F, F) {
    let t1245 = t55 * t109 * t342;
    let t1246 = t1243 * t1245;
    let t1247 = F::new(0.9743416666666667) * t1246;
    let t1249 = F::new(0.6495611111111111) * t349 * t947;
    (t1245, t1246, t1247, t1249)
}
