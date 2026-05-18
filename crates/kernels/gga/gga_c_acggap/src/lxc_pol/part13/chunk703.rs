//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 703/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk703<F: Float>(t22: F, t7508: F, t420: F, t56: F, t1072: F, t368: F, t7507: F, t1095: F, t1077: F, t137: F, t1083: F, t1089: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7510 = F::new(1.0) / t22 / t7508;
    let t7512 = t7510 * t56 * t420;
    let t7513 = t368 * t1072;
    let t7514 = t7512 * t7513;
    let t7515 = t7507 * t7514;
    let t7516 = F::new(0.42874018118069736972e-3) * t7515;
    let t7517 = t1095 * t1072;
    let t7518 = t7512 * t7517;
    let t7519 = t7507 * t7518;
    let t7520 = F::new(0.62896184579208304134e-3) * t7519;
    let t7521 = t137 * t1077;
    let t7523 = t1089 * t1083 * t7521;
    (t7510, t7512, t7513, t7514, t7516, t7517, t7518, t7520, t7523)
}
