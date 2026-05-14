//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 886/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk886<F: Float>(t15008: F, t1580: F, t13820: F, t1579: F, t4381: F, t4384: F, t1576: F, t4510: F, t13614: F, t397: F, t539: F, t535: F, t4370: F, t4502: F, t1572: F, t4416: F) -> (F, F, F, F, F, F, F, F) {
    let t15009 = t1580 * t15008;
    let t15011 = t1579 * t13820;
    let t15014 = t4381 * t4384;
    let t15047 = t4510 * t1576;
    let t15050 = t397 * t13614 * t539;
    let t15052 = 0.9994882620098509563e-2 * t535 * t15050;
    let t15056 = t4370 * t1576;
    let t15062 = t4502 * t1576;
    let t15064 = t1572 * t4416;
    (t15009, t15011, t15014, t15047, t15052, t15056, t15062, t15064)
}
