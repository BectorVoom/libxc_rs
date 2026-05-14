//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 857/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk857<F: Float>(t464: F, t7034: F, t2271: F, t2810: F, t2813: F, t2452: F, t410: F, t406: F, t4904: F, t889: F, t1212: F, t35: F, t4920: F, t893: F, t1224: F, t2484: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7035 = t7034 * t464;
    let t7036 = 0.36622894612013090108e-3 * t7035;
    let t7048 = 0.4726e1 * t2271 * t2810;
    let t7050 = 0.4726e1 * t2271 * t2813;
    let t7051 = t410 * t2452;
    let t7054 = t406 * t2452;
    let t7059 = t4904 * t889;
    let t7062 = t1212 * t35;
    let t7073 = t4920 * t893;
    let t7076 = t1224 * t35;
    let t7094 = t406 * t2484;
    (t7035, t7036, t7048, t7050, t7051, t7054, t7059, t7062, t7073, t7076, t7094)
}
