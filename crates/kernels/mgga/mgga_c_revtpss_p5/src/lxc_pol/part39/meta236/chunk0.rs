//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 908/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk908<F: Float>(t482: F, t5245: F, t371: F, t372: F, t1234: F, t1803: F, t225: F, t5219: F, t480: F, t3623: F, t4890: F) -> (F, F, F, F, F, F) {
    let t5318 = t482 * t5245;
    let t5320 = t371 * t372 * t5318;
    let t5323 = t1234 * t1803;
    let t5326 = t5219 * t225;
    let t5327 = t5326 * t480;
    let t5330 = t3623 * t4890;
    (t5318, t5320, t5323, t5326, t5327, t5330)
}
