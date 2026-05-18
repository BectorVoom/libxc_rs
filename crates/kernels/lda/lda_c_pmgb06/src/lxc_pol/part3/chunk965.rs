//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 965/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk965<F: Float>(t11381: F, t360: F, t11286: F, t35: F, t64: F, t1830: F, t2226: F, t2186: F, t1180: F, t776: F, t5793: F, t947: F) -> (F, F, F, F, F, F, F) {
    let t11382 = t360 * t11381;
    let t11385 = t35 * t64 * t11286;
    let t11388 = t2226 * t1830;
    let t11390 = t2186 * t1830;
    let t11391 = F::new(1.5156425925925925) * t11390;
    let t11392 = t1180 * t776;
    let t11393 = t360 * t11392;
    let t11395 = t5793 * t947;
    (t11382, t11385, t11388, t11391, t11392, t11393, t11395)
}
