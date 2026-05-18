//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1046/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1046<F: Float>(t3860: F, t4738: F, t10056: F, t2967: F, t743: F, t4776: F, t571: F, t2018: F, t3727: F, t1472: F, t4773: F, t4777: F) -> (F, F, F, F, F, F) {
    let t12251 = t4738 * t3860;
    let t12252 = F::new(32.0) / F::new(45.0) * t12251;
    let t12254 = t10056 * t743 * t2967;
    let t12257 = F::new(128.0) / F::new(27.0) * t571 * t4776 * t12254;
    let t12259 = F::new(4.0) / F::new(9.0) * t3727 * t2018;
    let t12261 = F::new(4.0) / F::new(9.0) * t1472 * t4773;
    let t12263 = F::new(32.0) / F::new(27.0) * t1472 * t4777;
    (t12252, t12254, t12257, t12259, t12261, t12263)
}
