//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1208/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1208<F: Float>(t14240: F, t4666: F, t571: F, t4680: F, t4794: F, t3394: F, t4738: F, t3399: F, t2011: F, t3727: F, t10605: F, t1944: F, t219: F) -> (F, F, F, F, F, F) {
    let t14242 = t571 * t14240 * t4666;
    let t14243 = F::new(64.0) / F::new(81.0) * t14242;
    let t14245 = t571 * t4794 * t4680;
    let t14246 = F::new(8.0) / F::new(27.0) * t14245;
    let t14248 = F::new(8.0) / F::new(15.0) * t4738 * t3394;
    let t14250 = F::new(8.0) / F::new(9.0) * t4738 * t3399;
    let t14252 = F::new(4.0) / F::new(15.0) * t3727 * t2011;
    let t14255 = t571 * t10605 * t219 * t1944;
    (t14243, t14246, t14248, t14250, t14252, t14255)
}
