//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1113/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1113<F: Float>(t3567: F, t8945: F, t26894: F, t29199: F, t3596: F, t37885: F, t2149: F, t1210: F, t13181: F, t3140: F, t1243: F, t2147: F, t44841: F, t7635: F, t45551: F, t473: F) -> (F, F, F, F, F, F, F) {
    let t97304 = t3567 * t8945;
    let t97308 = t26894 * t29199;
    let t97312 = t37885 * t3596;
    let t97313 = t2149 * t97312;
    let t97318 = t1210 * t29199;
    let t97346 = t3140 * t13181;
    let t97348 = t2149 * t97346 * t1243;
    let t97358 = t2147 * t44841 * t7635;
    let t97377 = t45551 * t473;
    (t97304, t97308, t97313, t97318, t97348, t97358, t97377)
}
