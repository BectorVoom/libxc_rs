//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 546/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk546<F: Float>(t5052: F, t5308: F, t1012: F, t3698: F, t5047: F, t482: F, t5245: F, t371: F, t372: F, t1234: F, t1803: F, t225: F, t5219: F, t480: F, t3623: F, t4890: F) -> (F, F, F, F, F, F, F) {
    let t5309 = t5308 * t5052;
    let t5312 = t1012 * t3698;
    let t5313 = t5312 * t5047;
    let t5318 = t482 * t5245;
    let t5320 = t371 * t372 * t5318;
    let t5323 = t1234 * t1803;
    let t5326 = t5219 * t225;
    let t5327 = t5326 * t480;
    let t5330 = t3623 * t4890;
    (t5309, t5313, t5320, t5323, t5326, t5327, t5330)
}
