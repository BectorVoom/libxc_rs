//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 187/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk187<F: Float>(t25: F, t596: F, t578: F, t582: F, t586: F, t590: F, t594: F, t88: F, t90: F, t29: F) -> (F, F, F) {
    let t598 = 6.0 * t25 * t596;
    let t599 = t578 - t582 + t586 - t590 + t594 - t598;
    let t602 = 1.0 / t90 / t88;
    let t603 = t29 * t602;
    (t599, t602, t603)
}
