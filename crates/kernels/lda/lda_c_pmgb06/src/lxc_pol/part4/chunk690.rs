//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 690/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk690<F: Float>(t1179: F, t206: F, t4068: F, t1830: F, t2060: F, t83: F, t208: F, t213: F, t1697: F, t97: F, t588: F, t1166: F, t579: F) -> (F, F, F, F, F, F, F, F) {
    let t4070 = F::new(0.001515438175925926) * t206 * t1179 * t4068;
    let t4075 = F::new(0.1005925925925926) * t1830 - F::new(0.5007407407407407) * t2060;
    let t4076 = t83 * t4075;
    let t4077 = t4076 * t208;
    let t4079 = t4077 * t213 / F::new(3.0);
    let t4080 = t1697 * t97;
    let t4082 = F::new(0.18233333333333332) * t4080 * t588;
    let t4087 = t1166 * t579;
    (t4070, t4075, t4076, t4077, t4079, t4080, t4082, t4087)
}
