//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 926/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk926<F: Float>(t1012: F, t1224: F, t5052: F, t3698: F, t5047: F, t482: F, t5245: F, t371: F, t372: F) -> (F, F, F, F, F, F) {
    let t5308 = t1012 * t1224;
    let t5309 = t5308 * t5052;
    let t5312 = t1012 * t3698;
    let t5313 = t5312 * t5047;
    let t5318 = t482 * t5245;
    let t5320 = t371 * t372 * t5318;
    (t5308, t5309, t5312, t5313, t5318, t5320)
}
