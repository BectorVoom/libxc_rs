//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 886/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk886<F: Float>(t1214: F, t1469: F, t5296: F, t1042: F, t3362: F, t3617: F, t4181: F, t1012: F, t1224: F, t5052: F, t3698: F, t5047: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5297 = t1469 * t1214;
    let t5298 = t5296 * t5297;
    let t5299 = t1042 * t5298;
    let t5302 = t3617 * t3362;
    let t5303 = t5302 * t4181;
    let t5304 = t1042 * t5303;
    let t5308 = t1012 * t1224;
    let t5309 = t5308 * t5052;
    let t5312 = t1012 * t3698;
    let t5313 = t5312 * t5047;
    (t5297, t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312, t5313)
}
