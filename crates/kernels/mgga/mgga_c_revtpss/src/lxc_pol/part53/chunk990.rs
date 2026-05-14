//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 990/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk990<F: Float>(t10309: F, t32801: F, t10301: F, t45963: F, t8736: F, t32805: F, t116: F, t32814: F, t2172: F, t7318: F, t32910: F, t571: F, t1464: F, t8766: F, t2045: F, t7690: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122893 = t10309 * t32801;
    let t122901 = t10301 * t32801;
    let t122911 = t45963 * t8736;
    let t122918 = t10309 * t32805;
    let t122950 = t32814 * t116;
    let t123122 = t7318 * t2172;
    let t123124 = t571 * t32910;
    let t123129 = t8766 * t1464;
    let t123131 = t7690 * t2045;
    (t122893, t122901, t122911, t122918, t122950, t123122, t123124, t123129, t123131)
}
