//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1333/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1333<F: Float>(t117294: F, t33098: F, t17945: F, t33097: F, t17966: F, t9704: F, t17057: F, t5317: F, t4581: F, t7413: F, t117268: F, t117270: F, t117272: F, t117274: F, t117276: F, t117278: F, t117280: F, t117282: F, t117284: F, t117286: F, t117288: F, t117290: F, t117292: F) -> (F, F, F, F, F, F) {
    let t117295 = t117294 * t33098;
    let t117297 = t33097 * t17945;
    let t117299 = t9704 * t17966;
    let t117301 = t17057 * t5317;
    let t117303 = t4581 * t7413;
    let t117305 = t117268 / 54.0 + 2.0 / 9.0 * t117270 - t117272 / 24.0 + t117274 / 12.0 + t117276 / 4.0 - t117278 / 12.0 + t117280 / 144.0 - t117282 / 8.0 + t117284 / 128.0 - t117286 / 48.0 + t117288 / 64.0 - t117290 / 288.0 - t117292 / 96.0 - t117295 / 3.0 + t117297 / 12.0 - t117299 / 24.0 - t117301 / 64.0 + t117303 / 9.0;
    (t117295, t117297, t117299, t117301, t117303, t117305)
}
