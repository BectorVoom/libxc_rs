//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1010/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1010<F: Float>(t3271: F, t42413: F, t11523: F, t11540: F, t2333: F, t3060: F, t795: F, t10997: F, t3275: F, t3229: F, t3276: F, t792: F, t8601: F, t12414: F, t10610: F, t3579: F, t40649: F) -> (F, F, F, F, F, F, F) {
    let t42415 = t42413 * t3271 / 4.0;
    let t42417 = t11523 * t11540 / 2.0;
    let t42418 = t2333 * t3060;
    let t42419 = t42418 * t795;
    let t42422 = 45.0 / 64.0 * t3275 * t10997 * t42419;
    let t42423 = t2333 * t3229;
    let t42424 = t42423 * t795;
    let t42427 = 5.0 / 16.0 * t3275 * t3276 * t42424;
    let t42428 = t8601 * t792;
    let t42431 = 5.0 / 16.0 * t3275 * t3276 * t42428;
    let t42432 = t12414 * t792;
    let t42435 = 15.0 / 8.0 * t10610 * t3276 * t42432;
    let t42437 = t3579 * t40649 / 2.0;
    (t42415, t42417, t42422, t42427, t42431, t42435, t42437)
}
