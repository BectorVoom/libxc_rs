//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1140/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1140<F: Float>(t108333: F, t108322: F, t108326: F, t108330: F, t108338: F, t108342: F, t108345: F, t108348: F, t97046: F, t97061: F, t97352: F, t108353: F, t108356: F, t108393: F, t108360: F, t108364: F, t108368: F, t108371: F, t108376: F, t108381: F, t108386: F, t108391: F, t108397: F) -> (F, F) {
    let t110151 = 4.0 / 9.0 * t108333;
    let t110158 = t97352 - 2.0 * t108322 + 4.0 / 3.0 * t108326 - 2.0 / 9.0 * t108330 - t110151 + 4.0 / 3.0 * t108338 + 2.0 / 3.0 * t108342 + t108345 / 27.0 + 2.0 / 3.0 * t108348 + 2.0 / 3.0 * t97046 + 16.0 / 27.0 * t97061;
    let t110159 = 4.0 / 9.0 * t108353;
    let t110160 = 4.0 / 9.0 * t108356;
    let t110169 = 4.0 / 27.0 * t108393;
    let t110171 = -t110159 - t110160 + 2.0 / 3.0 * t108360 + 4.0 * t108364 - 4.0 / 27.0 * t108368 - 4.0 / 9.0 * t108371 + t108376 / 6.0 + t108381 / 12.0 + t108386 / 12.0 - t108391 / 8.0 + t110169 + 10.0 / 81.0 * t108397;
    (t110158, t110171)
}
