//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1145/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1145<F: Float>(t1017: F, t11759: F, t11811: F, t11824: F, t25539: F, t3248: F, t3255: F, t7111: F, t7117: F, t93683: F, t93685: F, t93687: F, t93689: F, t93691: F, t93694: F, t93696: F, t93702: F, t93704: F) -> (F,) {
    let t93710 = -0.42874018118069736972e-3 * t7117 * t11811 - 0.17149607247227894789e-2 * t93683 - 0.85748036236139473944e-3 * t93685 - 0.11433071498151929859e-2 * t93687 + 0.17149607247227894789e-2 * t93689 + 11.0 / 108.0 * t93691 * t1017 - t93694 / 54.0 - t93696 / 432.0 - t25539 * t3248 / 36.0 - t25539 * t3255 / 27.0 + t93702 / 288.0 + t93704 / 216.0 + t7111 * t11759 / 288.0 + 7.0 / 648.0 * t7111 * t11824;
    (t93710,)
}
