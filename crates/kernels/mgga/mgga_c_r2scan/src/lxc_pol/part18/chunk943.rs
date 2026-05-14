//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 943/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk943<F: Float>(t321: F, t6100: F, t1266: F, t818: F, t826: F, t11056: F, t1271: F, t819: F, t1276: F, t3416: F, t6767: F, t1096: F, t19327: F, t6755: F, t19309: F, t113: F, t3268: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t37038 = t6100 * t321;
    let t37039 = 154.0 / 27.0 * t37038;
    let t37040 = t1266 * t818;
    let t37041 = t37040 * t826;
    let t37066 = t1271 * t11056;
    let t37074 = param_eta * t6100;
    let t37075 = t819 * t37074;
    let t37076 = 154.0 / 27.0 * t37075;
    let t37078 = t1276 * t11056 * t826;
    let t37204 = t6767 * t3416;
    let t37209 = t19327 * t1096;
    let t37223 = t6755 * t3416;
    let t37226 = t19309 * t1096;
    let t37271 = t97 * t3268 * t113;
    (t37039, t37040, t37041, t37066, t37076, t37078, t37204, t37209, t37223, t37226, t37271)
}
