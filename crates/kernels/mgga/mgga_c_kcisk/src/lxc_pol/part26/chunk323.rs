//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 323/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk323<F: Float>(t1056: F, t1471: F, t1472: F, t1402: F, t416: F, t140: F, t1429: F, t1434: F, t1460: F, t1469: F, t1470: F, t460: F, t476: F, t479: F, t467: F, t488: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t1474 = t1471 * t1472 * t1056;
    let t1477 = t416 * t1402;
    let t1481 = 0.619125e-2 * t1460 * t460 + 0.9286875e-2 * t476 * t1429 - 0.619125e-2 * t476 * t1434 - t1469 - 0.26531111111111111111e-1 * t1470 * t1474 - 0.39796666666666666666e-1 * t140 * t479 * t1477;
    let t1482 = t1481 * t467;
    let t1483 = t1482 * sigma0;
    let t1484 = t1483 * t488;
    (t1474, t1477, t1481, t1482, t1483, t1484)
}
