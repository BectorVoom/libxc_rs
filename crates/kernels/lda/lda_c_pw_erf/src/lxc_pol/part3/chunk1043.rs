//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1043/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1043<F: Float>(t3794: F, t4831: F, t1325: F, t1326: F, t3464: F, t789: F, t1976: F, t4829: F, t944: F, t1308: F, t4768: F, t571: F, t954: F) -> (F, F, F, F) {
    let t12211 = F::new(32.0) / F::new(15.0) * t3794 * t4831;
    let t12215 = F::new(8.0) / F::new(45.0) * t1325 * t1326 * t789 * t3464;
    let t12219 = F::new(16.0) / F::new(15.0) * t1325 * t4829 * t1976 * t944;
    let t12223 = F::new(4.0) / F::new(15.0) * t571 * t1308 * t4768 * t954;
    (t12211, t12215, t12219, t12223)
}
