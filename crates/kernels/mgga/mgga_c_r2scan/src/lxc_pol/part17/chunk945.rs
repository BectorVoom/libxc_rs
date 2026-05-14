//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 945/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk945<F: Float>(t11056: F, t1271: F, t6100: F, t819: F, t1276: F, t826: F, t113: F, t3268: F, t97: F, t10666: F, t1561: F, t3261: F, t122: F, t874: F, t3438: F, t10978: F, t10979: F, t2317: F) -> (F, F, F, F, F, F, F, F) {
    let t37066 = t1271 * t11056;
    let t37074 = param_eta * t6100;
    let t37075 = t819 * t37074;
    let t37078 = t1276 * t11056 * t826;
    let t37271 = t97 * t3268 * t113;
    let t37282 = t97 * t10666 * t113;
    let t37327 = t97 * t3261 * t1561;
    let t37355 = t874 * t122;
    let t37356 = t3438 * t37355;
    let t37358 = t10978 * t10979 * t2317 * t37356;
    (t37066, t37075, t37078, t37271, t37282, t37327, t37355, t37358)
}
