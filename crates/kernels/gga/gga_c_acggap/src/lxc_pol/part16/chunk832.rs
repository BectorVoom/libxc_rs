//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 832/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk832<F: Float>(t31391: F, t22: F, t30174: F, t420: F, t56: F, t7507: F, t7513: F, t174: F, t30779: F, t7322: F, t3125: F, t721: F, t1981: F, t2015: F, t1170: F, t31056: F) -> (F, F, F, F, F, F, F) {
    let t31392 = 0.1886885537376249124e-2 * t31391;
    let t31402 = 1.0 / t22 / t30174;
    let t31404 = t31402 * t56 * t420;
    let t31406 = t7507 * t31404 * t7513;
    let t31407 = 0.94322839859753421338e-2 * t31406;
    let t31419 = t7322 * t30779 * t174;
    let t31421 = t31419 * t3125 * t721;
    let t31428 = t2015 * t1981;
    let t31443 = t1170 * t31056;
    (t31392, t31404, t31407, t31419, t31421, t31428, t31443)
}
