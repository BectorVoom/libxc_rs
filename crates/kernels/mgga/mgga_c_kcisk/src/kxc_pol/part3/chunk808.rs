//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 808/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk808<F: Float>(t1340: F, t13431: F, t1339: F, t1218: F, t338: F, t1327: F, t3922: F, t3923: F, t408: F, t1219: F, t3729: F, t1286: F, t3283: F, t3485: F, t3484: F, t3482: F) -> (F, F, F, F, F) {
    let t13432 = t1340 * t13431;
    let t13433 = t1339 * t13432;
    let t13435 = t1218 * t1218;
    let t13436 = 1.0 / t13435;
    let t13437 = t338 * t13436;
    let t13438 = t3922 * t1327;
    let t13440 = 1.0 / t3923 / t408;
    let t13441 = t13438 * t13440;
    let t13448 = t3729 * t1219;
    let t13451 = t3283 * t1286;
    let t13452 = t3485 * t13451;
    let t13453 = t3484 * t13452;
    let t13454 = t3482 * t13453;
    (t13433, t13437, t13441, t13448, t13454)
}
