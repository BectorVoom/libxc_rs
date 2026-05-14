//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1076/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1076<F: Float>(t127336: F, t127340: F, t127341: F, t127346: F, t129455: F, t129457: F, t129459: F, t129461: F, t129463: F, t129465: F, t129468: F, t129471: F, t129473: F, t129467: F, t1936: F, t129470: F) -> (F, F, F) {
    let t129476 = -t129455 - 3.0 * t127336 - t127340 - 2.0 * t129457 - 2.0 * t129459 - 2.0 * t129461 - 2.0 * t129463 - 2.0 * t129465 - 2.0 * t129468 - 2.0 * t129471 - 2.0 * t129473 + 3.0 * t127341 - t127346;
    let t129478 = t129467 * t1936;
    let t129479 = t129470 * t1936;
    (t129476, t129478, t129479)
}
