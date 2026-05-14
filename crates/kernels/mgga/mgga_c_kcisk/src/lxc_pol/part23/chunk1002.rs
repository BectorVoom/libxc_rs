//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1002/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1002<F: Float>(t13561: F, t2141: F, t13565: F, t4101: F, t4100: F, t6119: F, t6101: F, t1224: F, t13524: F, t2075: F) -> (F, F, F) {
    let t20276 = t13561 * t2141;
    let t20277 = t13565 * t4101;
    let t20278 = t20276 * t20277;
    let t20281 = t4100 * t6119;
    let t20282 = t20281 * t6101;
    let t20292 = t1224 * t13524 * t2075;
    (t20278, t20282, t20292)
}
