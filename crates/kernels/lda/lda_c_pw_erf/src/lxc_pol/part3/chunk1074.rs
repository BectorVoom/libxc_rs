//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1074/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1074<F: Float>(t4753: F, t5363: F, t5367: F, t1472: F, t4930: F, t1403: F, t1466: F, t2065: F, t3667: F, t571: F, t10505: F, t799: F) -> (F, F, F, F, F) {
    let t12577 = t4753 * t5363;
    let t12578 = F::new(16.0) / F::new(15.0) * t12577;
    let t12580 = F::new(4.0) / F::new(5.0) * t4753 * t5367;
    let t12582 = F::new(12.0) / F::new(5.0) * t1472 * t4930;
    let t12587 = F::new(12.0) / F::new(5.0) * t571 * t1466 * t3667 * t2065 * t1403;
    let t12589 = F::new(8.0) / F::new(15.0) * t10505 * t799;
    (t12578, t12580, t12582, t12587, t12589)
}
