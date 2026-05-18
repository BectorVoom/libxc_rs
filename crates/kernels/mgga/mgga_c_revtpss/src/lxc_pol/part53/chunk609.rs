//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 609/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk609<F: Float>(t5296: F, t5297: F, t1042: F, t3362: F, t3617: F, t4181: F, t1012: F, t1224: F, t5052: F, t3698: F, t5047: F, t482: F, t5245: F) -> (F, F, F, F, F) {
    let t5298 = t5296 * t5297;
    let t5299 = t1042 * t5298;
    let t5302 = t3617 * t3362;
    let t5303 = t5302 * t4181;
    let t5304 = t1042 * t5303;
    let t5308 = t1012 * t1224;
    let t5309 = t5308 * t5052;
    let t5312 = t1012 * t3698;
    let t5313 = t5312 * t5047;
    let t5318 = t482 * t5245;
    (t5299, t5304, t5309, t5313, t5318)
}
