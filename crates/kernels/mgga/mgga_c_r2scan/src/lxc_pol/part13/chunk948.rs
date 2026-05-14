//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 948/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk948<F: Float>(t1277: F, t3366: F, t6661: F, t6651: F, t11056: F, t1271: F, t1276: F, t1289: F, t6100: F, t819: F, t826: F, t11153: F, t1348: F, t3416: F, t6767: F, t1096: F, t19327: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37055 = t6661 * t3366 * t1277;
    let t37063 = t6651 * t3366;
    let t37066 = t1271 * t11056;
    let t37069 = t1276 * t3366 * t1289;
    let t37074 = param_eta * t6100;
    let t37075 = t819 * t37074;
    let t37076 = 154.0 / 27.0 * t37075;
    let t37078 = t1276 * t11056 * t826;
    let t37199 = t1348 * t11153;
    let t37204 = t6767 * t3416;
    let t37209 = t19327 * t1096;
    (t37055, t37063, t37066, t37069, t37076, t37078, t37199, t37204, t37209)
}
