//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 480/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk480<F: Float>(t1054: F, t1058: F, t1010: F, t614: F, t1016: F, t140: F, t1011: F, t271: F, t905: F, t1071: F, t342: F, t1077: F, t384: F) -> (F, F, F, F, F, F, F) {
    let t3234 = t1054 * t1058;
    let t3241 = t614 * t1010;
    let t3244 = t140 * t1016;
    let t3245 = t1011 * t3244;
    let t3252 = F::new(1.0) / t271 / t905;
    let t3264 = t342 * t1071;
    let t3268 = F::new(1.0) / t1077 / t384;
    (t3234, t3241, t3244, t3245, t3252, t3264, t3268)
}
