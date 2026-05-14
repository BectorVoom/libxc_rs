//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 552/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk552<F: Float>(t1250: F, t482: F, t5284: F, t1042: F, t1038: F, t1802: F, t1244: F, t1241: F, t1121: F, t1263: F, t1214: F, t1469: F, t3362: F, t3617: F, t4181: F, t1012: F, t1224: F) -> (F, F, F, F, F, F) {
    let t5286 = t482 * t5284 * t1250;
    let t5287 = t1042 * t5286;
    let t5291 = t1802 * t1038;
    let t5292 = t1244 * t5291;
    let t5293 = t1241 * t5292;
    let t5296 = t1263 * t1121;
    let t5297 = t1469 * t1214;
    let t5298 = t5296 * t5297;
    let t5299 = t1042 * t5298;
    let t5302 = t3617 * t3362;
    let t5303 = t5302 * t4181;
    let t5304 = t1042 * t5303;
    let t5308 = t1012 * t1224;
    (t5287, t5291, t5293, t5299, t5304, t5308)
}
