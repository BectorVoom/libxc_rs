//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 860/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk860<F: Float>(t60: F, t123: F, t15251: F, t15253: F, t6: F, t120: F, t20: F, t114: F, t3050: F, t927: F, t3058: F, t397: F, t12630: F, t925: F, t3015: F, t896: F, t2994: F) -> (F, F, F, F, F, F, F) {
    let t124 = 0.0 < t60;
    let t15255 = t123 * t6 * t15251 * t15253;
    let t15258 = t120 * t20;
    let t15259 = t114 * t15258;
    let t15260 = t3050 * t927;
    let t15262 = t397 * t15260 * t3058;
    let t15268 = piecewise3(t124, t12630, -t12630);
    let t15270 = t123 * t925 * t15268;
    let t15274 = t3015 * t896;
    let t15278 = t2994 * t896;
    (t15255, t15259, t15262, t15268, t15270, t15274, t15278)
}
