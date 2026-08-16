//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 930/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk930<F: Float>(t11671: F, t3114: F, t127: F, t3206: F, t371: F, t3205: F, t11200: F, t225: F, t366: F, t11202: F, t373: F, t372: F) -> (F, F, F, F, F) {
    let t11933 = t3114 * t11671;
    let t11937 = t371 * t127 * t3206;
    let t11938 = t3205 * t11937;
    let t11940 = t11200 * t225;
    let t11941 = t11940 * t366;
    let t11942 = t373 * t11202;
    let t11944 = t371 * t372 * t11942;
    (t11933, t11938, t11940, t11941, t11944)
}
