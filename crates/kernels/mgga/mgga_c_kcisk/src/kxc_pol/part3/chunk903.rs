//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 903/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk903<F: Float>(t15039: F, t79: F, t534: F, t1567: F, t4509: F, t1576: F, t4510: F, t13614: F, t397: F, t539: F, t535: F, t1571: F, t4369: F, t4370: F, t14164: F, t20: F) -> (F, F, F, F, F, F, F) {
    let t15040 = t79 * t15039;
    let t15041 = t15040 * t534;
    let t15044 = t1567 * t4509;
    let t15047 = t4510 * t1576;
    let t15050 = t397 * t13614 * t539;
    let t15052 = 0.9994882620098509563e-2 * t535 * t15050;
    let t15053 = t4369 * t1571;
    let t15056 = t4370 * t1576;
    let t15058 = t14164 * t20;
    (t15041, t15044, t15047, t15052, t15053, t15056, t15058)
}
