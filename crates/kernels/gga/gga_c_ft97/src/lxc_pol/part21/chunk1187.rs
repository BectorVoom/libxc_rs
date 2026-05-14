//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1187/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1187<F: Float>(t100253: F, t116340: F, t116344: F, t116346: F, t116350: F, t116354: F, t116358: F, t116363: F, t116368: F, t116373: F, t116377: F, t92186: F, t100271: F, t100273: F, t116383: F, t116387: F, t116390: F, t116393: F, t116395: F, t116400: F, t116402: F, t116405: F, t116408: F, t92201: F) -> (F, F) {
    let t117079 = -t116340 / 9.0 + 2.0 / 27.0 * t116344 - t116346 / 27.0 - t116350 / 9.0 - t116354 / 3.0 + t116358 / 6.0 + t100253 + t116363 / 12.0 + 2.0 / 3.0 * t116368 + t116373 / 12.0 + t116377 / 9.0 + t92186;
    let t117090 = t116383 / 2.0 + 4.0 * t116387 + 8.0 / 9.0 * t116390 - 8.0 / 27.0 * t116393 + 2.0 / 27.0 * t116395 + 4.0 / 81.0 * t92201 - t116400 / 3.0 + t100271 + t100273 - 8.0 / 9.0 * t116402 + 4.0 / 9.0 * t116405 - 4.0 / 9.0 * t116408;
    (t117079, t117090)
}
