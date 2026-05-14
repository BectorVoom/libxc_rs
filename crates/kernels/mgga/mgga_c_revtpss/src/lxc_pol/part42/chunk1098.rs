//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1098/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1098<F: Float>(t45: F, t57: F, t18272: F, t18277: F, t18281: F, t4186: F, t4377: F, t606: F, t78: F, t10457: F, t5819: F, t2382: F, t5825: F, t4384: F, t81: F, t150: F, t190: F, t5944: F, t750: F, zeta_threshold: F) -> (F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t18285 = piecewise3(t151, 0.0, -8.0 / 27.0 * t18272 * t606 + 8.0 / 9.0 * t4377 * t4186 + 4.0 / 9.0 * t18277 * t606 + 4.0 / 3.0 * t78 * t18281);
    let t18286 = t10457 * t5819;
    let t18291 = t2382 * t5825;
    let t18297 = piecewise3(t155, 0.0, 8.0 / 27.0 * t18286 * t606 + 8.0 / 9.0 * t4384 * t4186 + 4.0 / 9.0 * t18291 * t606 - 4.0 / 3.0 * t81 * t18281);
    let t18298 = t18285 + t18297;
    let t18299 = t150 * t18298;
    let t18300 = t18299 * t190;
    let t18301 = t5944 * t750;
    (t18298, t18300, t18301)
}
