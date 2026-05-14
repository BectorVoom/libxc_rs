//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 370/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk370<F: Float>(t1258: F, t2113: F, t1268: F, t1271: F, t2120: F, t2123: F, t2126: F, t1266: F, t1275: F, t1234: F, t1264: F, t2115: F, t2129: F, t361: F, t374: F, t45: F) -> (F, F, F, F) {
    let t2133 = -t1258 - 0.92708333333333333333e-2 * t2113;
    let t2141 = 0.258925e1 * t2120 - t1268 - 0.301925e0 * t2113 + 0.16504875e0 * t2123 - t1271 - 0.16557e0 * t2126;
    let t2143 = t1266 * t2141 * t1275;
    let t2146 = -0.62182e-1 * t2115 * t361 + 1.0 * t1234 * t2129 + 0.19751789702565206229e-1 * t45 * t2133 * t374 - 0.58482233974552040708e0 * t1264 * t2143;
    (t2133, t2141, t2143, t2146)
}
