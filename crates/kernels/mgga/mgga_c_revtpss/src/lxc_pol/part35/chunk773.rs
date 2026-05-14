//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 773/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk773<F: Float>(t6461: F, t698: F, t6464: F, t6467: F, t6422: F, t689: F) -> (F, F, F, F) {
    let t20276 = t698 * t6461;
    let t20278 = t698 * t6464;
    let t20280 = t698 * t6467;
    let t20283 = t689 * t6422;
    (t20276, t20278, t20280, t20283)
}
