//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 462/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk462<F: Float>(t1015: F, t1469: F, t1012: F, t1647: F, t225: F, t366: F, t1651: F, t373: F, t372: F, t371: F) -> (F, F, F, F, F, F) {
    let t1655 = t1015 * t1469;
    let t1656 = t1012 * t1655;
    let t1659 = t1647 * t225;
    let t1660 = t1659 * t366;
    let t1663 = t373 * t1651;
    let t1664 = t372 * t1663;
    let t1665 = t371 * t1664;
    (t1655, t1656, t1659, t1660, t1663, t1665)
}
