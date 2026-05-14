//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 976/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk976<F: Float>(t39361: F, t2650: F, t546: F, t565: F, t10698: F, t2559: F, t10772: F, t10810: F, t2578: F, t1577: F, t2599: F, t2096: F, t2649: F, t571: F, t10769: F, t2547: F, t37764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39362 = 0.23115257973478049502e0 * t39361;
    let t39375 = t546 * t2650;
    let t39378 = t565 * t2650;
    let t39395 = t10698 * t2559;
    let t39396 = 0.12805040077930161442e0 * t39395;
    let t39400 = t10772 * t10810 * t2578;
    let t39401 = 0.69345773920434148506e0 * t39400;
    let t39403 = t1577 * t10810 * t2599;
    let t39404 = 0.46230515946956099004e0 * t39403;
    let t39409 = t571 * t2649 * t2096;
    let t39410 = t39409 * t10769;
    let t39411 = 0.47609969197673950972e-2 * t39410;
    let t39420 = t37764 * t2547;
    (t39362, t39375, t39378, t39396, t39401, t39404, t39409, t39411, t39420)
}
