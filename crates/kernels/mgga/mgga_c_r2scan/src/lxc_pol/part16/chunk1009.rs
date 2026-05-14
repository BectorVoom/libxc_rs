//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1009/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1009<F: Float>(t14160: F, t3245: F, t3270: F, t3269: F, t2850: F, t6967: F, t3263: F, t3275: F, t12574: F, t37292: F, t3262: F, t3574: F, t40324: F, t106: F, t8691: F, t97: F) -> (F, F, F, F, F) {
    let t42399 = t14160 * t3245;
    let t42400 = t3270 * t42399;
    let t42402 = t3269 * t42400 / 2.0;
    let t42403 = t6967 * t2850;
    let t42405 = t3275 * t3263 * t42403;
    let t42408 = 45.0 / 64.0 * t3275 * t37292 * t12574;
    let t42411 = 3.0 / 2.0 * t3262 * t40324 * t3574;
    let t42413 = t97 * t106 * t8691;
    (t42402, t42405, t42408, t42411, t42413)
}
