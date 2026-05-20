//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1259/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1259<F: Float>(t93303: F, t93314: F, t25386: F, t93280: F, t93282: F, t786: F, t860: F, t25410: F, t25413: F, t7064: F, t93150: F, t25375: F, t93311: F) -> (F, F, F, F, F, F) {
    let t93315 = t93314 * t93303;
    let t93317 = t25386 * t93280;
    let t93318 = t93317 * t93282;
    let t93320 = t786 * t860;
    let t93321 = t93320 * t25410;
    let t93322 = t93321 * t25413;
    let t93324 = t7064 * t93150;
    let t93326 = t25375 * t93311;
    (t93315, t93318, t93320, t93322, t93324, t93326)
}
