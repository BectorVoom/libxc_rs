//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 982/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk982<F: Float>(t957: F, t682: F, t696: F, t978: F, t1092: F, t1105: F, t1090: F, t1108: F, t1065: F, t1089: F, t248: F, t3767: F, t638: F) -> (F, F, F, F, F, F) {
    let t8522 = t957 * t957;
    let t8526 = F::new(3.5089341735807875) * t696 * t978 * t8522 * t682;
    let t8529 = t1105 * t1092;
    let t8531 = t1108 * t1090;
    let t8534 = t248 * t1089 * t1065;
    let t8538 = t638 * t3767;
    (t8522, t8526, t8529, t8531, t8534, t8538)
}
