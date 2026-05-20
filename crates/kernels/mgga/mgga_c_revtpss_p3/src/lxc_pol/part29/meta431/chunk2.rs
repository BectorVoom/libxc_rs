//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1596/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1596<F: Float>(t1284: F, t5219: F, t3624: F, t1225: F, t13312: F, t1012: F, t1230: F, t5390: F, t12879: F, t1715: F, t247: F, t1261: F) -> (F, F, F, F) {
    let t17400 = t5219 * t1284;
    let t17401 = t17400 * t3624;
    let t17404 = t1225 * t13312;
    let t17405 = t1012 * t17404;
    let t17412 = t1230 * t5390;
    let t17416 = t247 * t12879 * t1715;
    let t17417 = t1261 * t17416;
    (t17401, t17405, t17412, t17417)
}
