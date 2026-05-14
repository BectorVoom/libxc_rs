//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 695/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk695<F: Float>(t747: F, t79: F, t6684: F, t7429: F, t1931: F, t2572: F, t7069: F, t746: F, t1948: F, t5322: F, t6702: F, t5321: F, t7407: F, t7411: F, t7414: F, t7416: F, t7418: F, t7420: F, t7422: F, t7425: F, t7427: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7430 = t79 * t747;
    let t7431 = t7430 * t6684;
    let t7432 = t7429 * t7431;
    let t7434 = t1931 * t2572;
    let t7436 = t747 * t7069;
    let t7437 = t746 * t7436;
    let t7438 = t1948 * t7437;
    let t7440 = t5322 * t6702;
    let t7441 = t5321 * t7440;
    let t7443 = -t7407 / 16.0 - t7411 / 256.0 + t7414 / 36.0 - t7416 / 192.0 + t7418 / 48.0 - t7420 / 192.0 - t7422 / 16.0 + t7425 / 6.0 - t7427 / 6.0 - t7432 / 128.0 + t7434 / 24.0 + t7438 / 256.0 + t7441 / 192.0;
    (t7430, t7431, t7432, t7434, t7437, t7438, t7440, t7441, t7443)
}
