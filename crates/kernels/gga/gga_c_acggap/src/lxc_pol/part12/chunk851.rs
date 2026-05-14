//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 851/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk851<F: Float>(t310: F, t7506: F, t7514: F, t7518: F, t22: F, t30174: F, t420: F, t56: F, t7507: F, t7513: F, t174: F, t30779: F, t7322: F, t3125: F, t721: F, t7447: F, t7819: F) -> (F, F, F, F, F, F, F) {
    let t31388 = t310 * t7506;
    let t31389 = t31388 * t7514;
    let t31391 = t31388 * t7518;
    let t31402 = 1.0 / t22 / t30174;
    let t31404 = t31402 * t56 * t420;
    let t31406 = t7507 * t31404 * t7513;
    let t31419 = t7322 * t30779 * t174;
    let t31421 = t31419 * t3125 * t721;
    let t31426 = t7447 * t7819;
    (t31389, t31391, t31404, t31406, t31419, t31421, t31426)
}
