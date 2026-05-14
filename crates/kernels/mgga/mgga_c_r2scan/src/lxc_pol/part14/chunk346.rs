//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 346/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk346<F: Float>(t322: F, t1339: F, t1348: F, t1306: F, t1308: F, t1336: F, t1338: F, t1343: F, t352: F, t855: F, t410: F, t458: F) -> (F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t1349 = t1348 * t1339;
    let t1353 = piecewise5(t323, t1306 + t1308, t331, t1336, -0.21e1 * t1338 * t1339 * t352 - 0.105e1 * t855 * t1343 * t352 - 0.1575e1 * t1349 * t352);
    let t1355 = t410 * t458;
    let t1356 = 8.0 * t1355;
    (t1353, t1356)
}
