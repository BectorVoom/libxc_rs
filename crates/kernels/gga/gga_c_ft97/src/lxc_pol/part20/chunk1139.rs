//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1139/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1139<F: Float>(t108249: F, t108260: F, t108262: F, t108247: F, t108253: F, t108258: F, t108266: F, t108270: F, t96975: F, t96983: F, t96985: F, t97003: F, t108275: F, t108278: F, t108282: F, t108284: F, t108288: F, t108291: F, t108295: F, t108299: F, t108303: F, t108308: F, t108314: F, t97022: F) -> (F, F) {
    let t110125 = t108249 / 27.0;
    let t110128 = t108260 / 9.0;
    let t110129 = 2.0 / 3.0 * t108262;
    let t110132 = t96975 / 9.0 + 4.0 / 81.0 * t96983 + t96985 / 27.0 - t97003 / 81.0 + t108247 / 27.0 - t110125 + t108253 / 18.0 + t108258 / 9.0 + t110128 + t110129 - 2.0 * t108266 - 2.0 * t108270;
    let t110144 = -t108275 + 4.0 / 27.0 * t108278 + 2.0 / 27.0 * t108282 + 2.0 / 81.0 * t108284 - t97022 / 54.0 + 2.0 / 3.0 * t108288 + 8.0 / 27.0 * t108291 - t108295 / 36.0 - t108299 / 54.0 - t108303 / 18.0 - t108308 / 36.0 + t108314 / 9.0;
    (t110132, t110144)
}
