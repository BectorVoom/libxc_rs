//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 660/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk660<F: Float>(t5321: F, t7440: F, t7407: F, t7411: F, t7414: F, t7416: F, t7418: F, t7420: F, t7422: F, t7425: F, t7427: F, t7432: F, t7434: F, t7438: F, t7404: F) -> (F, F) {
    let t7441 = t5321 * t7440;
    let t7443 = -t7407 / 16.0 - t7411 / 256.0 + t7414 / 36.0 - t7416 / 192.0 + t7418 / 48.0 - t7420 / 192.0 - t7422 / 16.0 + t7425 / 6.0 - t7427 / 6.0 - t7432 / 128.0 + t7434 / 24.0 + t7438 / 256.0 + t7441 / 192.0;
    let t7444 = t7404 + t7443;
    (t7441, t7444)
}
