//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 917/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk917<F: Float>(t11775: F, t3096: F, t1024: F, t3230: F, t11213: F, t225: F, t366: F, t11223: F, t1053: F, t3223: F, t3215: F, t3224: F, t1011: F, t1028: F, t11753: F, t11756: F, t11759: F, t11763: F, t11767: F, t11774: F, t3208: F, t3211: F, t3220: F, t3238: F, t3241: F) -> (F, F, F, F) {
    let t11776 = t11775 * t3096;
    let t11779 = t1024 * t3230;
    let t11782 = t11213 * t225;
    let t11783 = t11782 * t366;
    let t11788 = t11223 * t225;
    let t11789 = t11788 * t366;
    let t11792 = t3223 * t1053;
    let t11795 = t3224 * t3215;
    let t11799 = t11753 / 288.0 + t11756 / 216.0 + t1011 * t11759 / 288.0 - t11763 / 144.0 + t1011 * t11767 / 48.0 + t3241 * t3238 / 18.0 - 0.85748036236139473944e-3 * t11774 * t11776 - 0.21722835846488666732e-1 * t11779 * t1028 - 0.64311027177104605458e-3 * t11783 * t1028 - 0.64311027177104605458e-3 * t3224 * t3220 + 0.12862205435420921092e-2 * t11789 * t3208 + 0.68598428988911579154e-2 * t11792 * t1028 - 0.85748036236139473944e-3 * t11795 + 0.34299214494455789577e-2 * t3211 * t3220;
    (t11776, t11782, t11788, t11799)
}
