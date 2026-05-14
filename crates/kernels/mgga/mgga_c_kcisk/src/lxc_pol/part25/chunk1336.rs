//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1336/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1336<F: Float>(t5336: F, t6719: F, t17914: F, t1873: F, t17885: F, t9704: F, t117306: F, t117308: F, t117311: F, t117313: F, t117315: F, t117317: F, t117319: F, t117321: F, t117323: F, t117325: F, t117328: F, t117330: F, t117332: F, t117335: F, t117337: F) -> (F, F, F, F) {
    let t117339 = t6719 * t5336;
    let t117341 = t1873 * t17914;
    let t117343 = t9704 * t17885;
    let t117345 = t117306 / 4.0 + t117308 / 16.0 + t117311 / 8.0 + t117313 / 128.0 + t117315 / 128.0 - t117317 / 12.0 - t117319 / 16.0 + t117321 / 24.0 - t117323 / 12.0 - t117325 / 12.0 + t117328 / 12.0 - t117330 / 36.0 - t117332 / 64.0 - t117335 / 144.0 + t117337 / 64.0 - t117339 / 96.0 - t117341 / 24.0 - t117343 / 24.0;
    (t117339, t117341, t117343, t117345)
}
