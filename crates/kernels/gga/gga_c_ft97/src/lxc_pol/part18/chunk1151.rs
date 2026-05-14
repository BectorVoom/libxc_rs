//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1151/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1151<F: Float>(t100271: F, t100273: F, t100277: F, t100283: F, t100288: F, t92186: F, t92191: F, t92194: F, t92201: F, t92218: F, t92237: F, t92239: F, t23008: F, t25896: F, t458: F, t23054: F, t25934: F) -> (F, F, F, F) {
    let t100290 = t92186 + 2.0 / 27.0 * t92191 + 2.0 / 3.0 * t92194 + 8.0 / 81.0 * t92201 + t92218 / 27.0 + t100271 + t100273 - t100277 / 6.0 - t92237 / 81.0 - t92239 / 54.0 - t100283 / 3.0 + t100288 / 9.0;
    let t100292 = t23008 * t458 * t25896;
    let t100293 = t100292 / 12.0;
    let t100294 = t23054 * t25934;
    (t100290, t100292, t100293, t100294)
}
