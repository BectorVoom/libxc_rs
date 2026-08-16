//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1305/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1305<F: Float>(t1398: F, t215: F, t268: F, t543: F, t4101: F, t2453: F, t4100: F, t281: F, t68: F, t1357: F, t4078: F, t689: F) -> (F, F, F, F) {
    let t10136 = t268 * t215 * t1398 * t543;
    let t10137 = t4101 * t10136;
    let t10139 = t2453 * t4100;
    let t10142 = t281 * t68 * t1398 * t543;
    let t10143 = t10139 * t10142;
    let t10150 = t1357 * t4078;
    let t10151 = t689 * t10150;
    (t10137, t10139, t10143, t10151)
}
