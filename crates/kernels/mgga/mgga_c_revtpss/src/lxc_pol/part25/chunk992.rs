//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 992/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk992<F: Float>(t1042: F, t13075: F, t1263: F, t3568: F, t1122: F, t247: F, t3372: F, t3634: F, t1261: F, t3368: F, t3636: F, t3647: F, t12646: F, t3719: F, t3367: F, t414: F) -> (F, F, F, F, F, F, F) {
    let t13076 = t1042 * t13075;
    let t13079 = t1263 * t3568;
    let t13080 = t13079 * t1122;
    let t13081 = t1042 * t13080;
    let t13085 = t247 * t3634 * t3372;
    let t13086 = t1261 * t13085;
    let t13089 = t247 * t3634 * t3368;
    let t13090 = t1261 * t13089;
    let t13092 = t3647 * t3636;
    let t13095 = t247 * t3719 * t12646;
    let t13099 = 1.0 / t414 / t3367;
    (t13076, t13081, t13086, t13090, t13092, t13095, t13099)
}
