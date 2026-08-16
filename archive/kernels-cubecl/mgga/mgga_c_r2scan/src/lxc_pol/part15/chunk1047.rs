//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1047/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1047<F: Float>(t3270: F, t37312: F, t10913: F, t498: F, t1561: F, t3261: F, t97: F, t1065: F, t1234: F, t105: F, t1550: F, t122: F, t874: F) -> (F, F, F, F, F, F) {
    let t37313 = t3270 * t37312;
    let t37318 = t498 * t10913;
    let t37327 = t97 * t3261 * t1561;
    let t37341 = t1065 * t1234;
    let t37342 = t3270 * t37341;
    let t37346 = t97 * t105 * t1550;
    let t37355 = t874 * t122;
    (t37313, t37318, t37327, t37342, t37346, t37355)
}
