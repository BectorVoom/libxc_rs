//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1363/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1363<F: Float>(t247: F, t2858: F, t3109: F, t1063: F, t140: F, t3247: F, t1011: F, t3254: F, t3237: F, t245: F, t3089: F, t3088: F) -> (F, F, F, F, F, F) {
    let t11744 = t247 * t3109 * t2858;
    let t11745 = t1063 * t11744;
    let t11752 = t140 * t3247;
    let t11753 = t1011 * t11752;
    let t11755 = t140 * t3254;
    let t11756 = t1011 * t11755;
    let t11762 = t140 * t3237;
    let t11763 = t1011 * t11762;
    let t11772 = t3089 * t245;
    let t11773 = t3088 * t11772;
    (t11745, t11753, t11756, t11763, t11772, t11773)
}
