//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1063/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1063<F: Float>(t29270: F, t3332: F, t6165: F, t10856: F, t9273: F, t39887: F, t39895: F, t43376: F, t43379: F, t43381: F, t43384: F, t43387: F, t43390: F, t43393: F, t43396: F, t11736: F, t11744: F) -> (F, F) {
    let t43399 = t6165 * t3332 * t29270;
    let t43401 = t10856 * t9273;
    let t43403 = -t39887 + 0.13099107994629972538e-1 * t43376 - 0.87327386630866483584e-2 * t43379 - 0.13099107994629972538e-1 * t43381 - 0.13099107994629972538e-1 * t43384 - 0.13099107994629972538e-1 * t43387 + 0.26198215989259945075e-1 * t43390 + 0.13099107994629972538e0 * t43393 - 0.5239643197851989015e-1 * t43396 - 0.65495539973149862688e-2 * t43399 - 0.97574405393827830187e-2 * t43401 - t39895;
    let t43407 = t11744 * t11736;
    (t43403, t43407)
}
