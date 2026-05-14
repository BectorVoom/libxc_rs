//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 444/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk444<F: Float>(t357: F, t905: F, t1065: F, t126: F, t1086: F, t994: F, t3090: F, t373: F, t66: F, t828: F) -> (F, F, F, F, F, F) {
    let t3094 = t357 * t905;
    let t3109 = t126 * t1065;
    let t3114 = t994 * t1086;
    let t3115 = t3114 * t3090;
    let t3116 = t66 * t373;
    let t3117 = t828 * t3116;
    (t3094, t3109, t3114, t3115, t3116, t3117)
}
