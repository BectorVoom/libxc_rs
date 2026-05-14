//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 424/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk424<F: Float>(t264: F, t1099: F, t281: F, t259: F, t1128: F, t278: F, t2925: F, t67: F, t10: F, t1102: F, t119: F, t142: F, t260: F, t261: F, t116: F, t1111: F, t1118: F) -> (F, F, F, F, F, F, F, F, F) {
    let t265 = t264 < -0.66725e-1;
    let t3372 = 1.0 / t1099 / t281;
    let t3373 = t259 * t3372;
    let t3374 = t1128 * t1128;
    let t3375 = t278 * t278;
    let t3376 = 1.0 / t3375;
    let t3377 = t3374 * t3376;
    let t3380 = t67 * t2925;
    let t3391 = piecewise3(t265, 0.0, 10.0 / 9.0 * t260 * t3380 * t10 - 20.0 / 27.0 * t260 * t1102 * t142 + 40.0 / 81.0 * t260 * t261 * t119);
    let t3392 = t3391 * t116;
    let t3399 = t1111 * t1118;
    (t3372, t3373, t3374, t3375, t3376, t3377, t3380, t3392, t3399)
}
