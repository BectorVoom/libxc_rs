//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1047/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1047<F: Float>(t3366: F, t6651: F, t11056: F, t1271: F, t1276: F, t1289: F, t6100: F, t819: F, t826: F, t113: F, t3268: F, t97: F) -> (F, F, F, F, F, F) {
    let t37063 = t6651 * t3366;
    let t37066 = t1271 * t11056;
    let t37069 = t1276 * t3366 * t1289;
    let t37074 = param_eta * t6100;
    let t37075 = t819 * t37074;
    let t37078 = t1276 * t11056 * t826;
    let t37271 = t97 * t3268 * t113;
    (t37063, t37066, t37069, t37075, t37078, t37271)
}
