//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1340/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1340<F: Float>(t11236: F, t33120: F, t7431: F, t17852: F, t34368: F, t18170: F, t9704: F, t61353: F, t748: F, t10375: F, t2587: F, t2454: F, t5277: F, t9705: F, t17894: F, t33121: F) -> (F, F, F, F, F, F, F) {
    let t117390 = t11236 * t33120 * t7431;
    let t117392 = t34368 * t17852;
    let t117394 = t9704 * t18170;
    let t117396 = t61353 * t748;
    let t117398 = t10375 * t2587;
    let t117400 = t5277 * t2454;
    let t117401 = t117400 * t9705;
    let t117403 = t33121 * t17894;
    (t117390, t117392, t117394, t117396, t117398, t117401, t117403)
}
