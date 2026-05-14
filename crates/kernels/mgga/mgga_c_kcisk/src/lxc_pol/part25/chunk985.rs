//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 985/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk985<F: Float>(t10937: F, t10941: F, t10944: F, t10947: F, t11030: F, t11033: F, t17400: F, t17402: F, t17405: F, t17408: F, t17412: F, t1224: F, t16026: F, t1697: F) -> (F, F) {
    let t17414 = -0.26574814814814814816e0 * t10937 + 0.66437037037037037038e-1 * t10941 - 0.19931111111111111111e0 * t10944 + 0.99655555555555555557e-1 * t10947 - t11030 - t11033 - t17400 + 0.13287407407407407408e0 * t17402 - 0.19931111111111111111e0 * t17405 - 0.33218518518518518518e0 * t17408 + 0.79724444444444444445e0 * t17412;
    let t17417 = t1224 * t1697 * t16026;
    (t17414, t17417)
}
