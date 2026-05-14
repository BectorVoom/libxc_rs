//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1057/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1057<F: Float>(t24208: F, t5321: F, t24183: F, t24185: F, t24187: F, t24191: F, t24193: F, t24195: F, t24197: F, t24200: F, t24204: F, t24206: F, t17936: F, t7299: F, t22952: F, t719: F) -> (F, F, F, F) {
    let t24209 = t5321 * t24208;
    let t24211 = -t24183 / 48.0 + t24185 / 24.0 + t24187 / 128.0 + t24191 / 36.0 + t24193 / 96.0 + t24195 / 12.0 + t24197 / 8.0 - 19.0 / 108.0 * t24200 - t24204 / 16.0 - 2.0 / 9.0 * t24206 + t24209 / 192.0;
    let t24214 = t17936 * t7299;
    let t24216 = t719 * t22952;
    (t24209, t24211, t24214, t24216)
}
