//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 425/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk425<F: Float>(t259: F, t20: F, t918: F, t268: F, t1120: F, t272: F, t1123: F, t397: F, t3366: F, t1121: F, t1111: F, t1119: F, t1125: F, t119: F, t275: F, t3042: F, t3392: F, t3399: F) -> (F, F, F, F, F, F, F, F) {
    let t270 = 0.0 < t259;
    let t3405 = t918 * t20;
    let t3406 = t268 * t3405;
    let t3410 = 1.0 / t1120 / t272;
    let t3411 = t1123 * t1123;
    let t3413 = t397 * t3410 * t3411;
    let t3417 = piecewise3(t270, t3366, -t3366);
    let t3419 = t397 * t1121 * t3417;
    let t3422 = 0.5397236614853195164e-1 * t3392 * t119 * t275 - 0.25187104202648244098e0 * t1111 * t918 * t275 - 0.10794473229706390328e0 * t3399 * t1125 + 0.41978507004413740163e0 * t268 * t3042 * t275 + 0.25187104202648244098e0 * t3406 * t1125 + 0.10794473229706390328e0 * t1119 * t3413 - 0.5397236614853195164e-1 * t1119 * t3419;
    (t3405, t3406, t3410, t3411, t3413, t3417, t3419, t3422)
}
