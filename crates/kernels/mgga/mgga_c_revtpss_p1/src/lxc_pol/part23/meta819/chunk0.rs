//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2667/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2667<F: Float>(t11277: F, t19826: F, t16163: F, t4879: F, t1063: F, t19681: F, t3172: F, t11710: F, t19625: F, t4899: F, t19687: F, t15772: F, t4834: F) -> (F, F, F, F, F, F) {
    let t65618 = t11277 * t19826;
    let t65627 = t4879 * t16163;
    let t65630 = t1063 * t3172 * t19681;
    let t65637 = t4899 * t11710 * t19625;
    let t65650 = t1063 * t3172 * t19687;
    let t65689 = t4834 * t15772;
    (t65618, t65627, t65630, t65637, t65650, t65689)
}
