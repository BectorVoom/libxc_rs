//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 598/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk598<F: Float>(t371: F, t372: F, t5318: F, t1234: F, t1803: F, t225: F, t5219: F, t480: F, t3623: F, t4890: F, t3782: F, t1794: F, t3153: F) -> (F, F, F, F, F, F, F) {
    let t5320 = t371 * t372 * t5318;
    let t5323 = t1234 * t1803;
    let t5326 = t5219 * t225;
    let t5327 = t5326 * t480;
    let t5330 = t3623 * t4890;
    let t5331 = t3782 * t5330;
    let t5332 = t1794 * t3153;
    (t5320, t5323, t5326, t5327, t5330, t5331, t5332)
}
