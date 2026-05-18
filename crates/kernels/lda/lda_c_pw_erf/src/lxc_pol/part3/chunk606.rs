//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 606/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk606<F: Float>(t3455: F, t496: F, t1234: F, t511: F, t1280: F, t1298: F, t1302: F, t2960: F) -> (F, F, F, F, F, F) {
    let t3457 = F::new(4.0) / F::new(5.0) * t3455 * t496;
    let t3458 = t511 * t1234;
    let t3459 = F::new(8.0) / F::new(15.0) * t3458;
    let t3461 = F::new(2.0) / F::new(5.0) * t511 * t1280;
    let t3463 = F::new(4.0) / F::new(5.0) * t1298 * t1302;
    let t3464 = F::new(3.0) * t2960;
    (t3457, t3458, t3459, t3461, t3463, t3464)
}
