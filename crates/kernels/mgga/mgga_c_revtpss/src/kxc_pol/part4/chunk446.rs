//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 446/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk446<F: Float>(t1469: F, t60: F, t1474: F, t1480: F, t44: F, t56: F, t61: F, t626: F, t38: F, t633: F, t637: F, t77: F, t1471: F, t71: F, t85: F) -> (F, F, F, F, F, F) {
    let t1483 = t60 * t1469;
    let t1486 = 5.0 / 6.0 * t44 * t1474 - 8.0 / 3.0 * t1480 * t61 - 5.0 / 6.0 * t56 * t1483 + t626;
    let t1487 = t38 * t1486;
    let t1490 = t633 * t1469;
    let t1491 = t637 * t1469;
    let t1493 = -4.0 / 3.0 * t1490 + 4.0 / 3.0 * t1491;
    let t1494 = t77 * t1493;
    let t1497 = -t1471 * t85 / 12.0 + t1487 * t85 / 24.0 + t71 * t1494 / 24.0;
    (t1486, t1487, t1490, t1491, t1494, t1497)
}
