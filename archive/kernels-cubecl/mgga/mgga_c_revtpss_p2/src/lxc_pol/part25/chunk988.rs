//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 988/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk988<F: Float>(t3182: F, t828: F, t2852: F, t357: F, t2251: F, t3093: F, t3109: F, t3096: F, t3091: F, t1020: F, t3105: F, t247: F, t2862: F) -> (F, F, F, F, F) {
    let t11703 = t828 * t3182;
    let t11704 = t357 * t2852;
    let t11705 = t11704 * t2251;
    let t11706 = t3093 * t11705;
    let t11707 = t11703 * t11706;
    let t11710 = t828 * t3109;
    let t11711 = t11710 * t3096;
    let t11712 = t3091 * t11711;
    let t11714 = t1020 * t3105;
    let t11722 = t247 * t3109 * t2862;
    (t11707, t11711, t11712, t11714, t11722)
}
