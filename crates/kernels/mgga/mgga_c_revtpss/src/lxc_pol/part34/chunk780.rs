//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 780/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk780<F: Float>(t3154: F, t357: F, t11249: F, t905: F, t3182: F, t828: F, t3109: F, t126: F, t3181: F, t221: F, t346: F, t68: F, t345: F, t1014: F, t2852: F, t245: F, t3089: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11631 = t3154 * t357;
    let t11632 = t11249 * t11631;
    let t11660 = t3154 * t905;
    let t11703 = t828 * t3182;
    let t11710 = t828 * t3109;
    let t11725 = t126 * t3181;
    let t11735 = t221 * t68 * t346;
    let t11737 = 5.0 / 1296.0 * t345 * t11735;
    let t11765 = t1014 * t2852;
    let t11772 = t3089 * t245;
    (t11631, t11632, t11660, t11703, t11710, t11725, t11735, t11737, t11765, t11772)
}
