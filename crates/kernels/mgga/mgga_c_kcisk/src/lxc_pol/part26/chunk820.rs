//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 820/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk820<F: Float>(t1390: F, t382: F, t1337: F, t1404: F, t1336: F, t140: F, t1299: F, t3483: F, t1412: F, t453: F, t1450: F, t3795: F, t10500: F, t472: F, t1218: F, t338: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13293 = t382 * t1390;
    let t13304 = t1337 * t1404;
    let t13306 = t140 * t1336 * t13304;
    let t13311 = t3483 * t1299;
    let t13327 = t1412 * t1412;
    let t13328 = 1.0 / t13327;
    let t13329 = t453 * t13328;
    let t13330 = t13329 * sigma0;
    let t13377 = t3795 * t1450;
    let t13399 = t10500 * t472;
    let t13400 = 0.73697530864197530862e-3 * t13399;
    let t13435 = t1218 * t1218;
    let t13436 = 1.0 / t13435;
    let t13437 = t338 * t13436;
    (t13293, t13306, t13311, t13328, t13329, t13330, t13377, t13399, t13400, t13435, t13436, t13437)
}
