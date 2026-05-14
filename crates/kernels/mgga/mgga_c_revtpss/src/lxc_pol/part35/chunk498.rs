//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 498/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk498<F: Float>(t1038: F, t1802: F, t1244: F, t1241: F, t1121: F, t1263: F, t3362: F, t3617: F, t1012: F, t1224: F, t3698: F, t1234: F, t1803: F, t225: F, t5219: F) -> (F, F, F, F, F, F, F, F) {
    let t5291 = t1802 * t1038;
    let t5292 = t1244 * t5291;
    let t5293 = t1241 * t5292;
    let t5296 = t1263 * t1121;
    let t5302 = t3617 * t3362;
    let t5308 = t1012 * t1224;
    let t5312 = t1012 * t3698;
    let t5323 = t1234 * t1803;
    let t5326 = t5219 * t225;
    (t5292, t5293, t5296, t5302, t5308, t5312, t5323, t5326)
}
