//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1002/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1002<F: Float>(t2923: F, t910: F, t287: F, t2922: F, t275: F, t11132: F, t2912: F, t698: F, t240: F, t624: F, t281: F, t283: F, t2909: F, t3252: F, t276: F, t285: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11294 = t910 * t2923;
    let t11298 = 1.0 / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11304 = 28.0 / 27.0 * t11132;
    let t11326 = t698 * t2912;
    let t11334 = 0.93011851851851851854e0 * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = 0.36514074074074074075e0 * t11337;
    let t11339 = t698 * t2909;
    let t11341 = t240 * t3252;
    let t11354 = 1.0 / t276 / t285 / 4.0;
    (t11294, t11299, t11304, t11326, t11334, t11335, t11337, t11338, t11339, t11341, t11354)
}
