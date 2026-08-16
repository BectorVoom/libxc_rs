//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 944/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk944<F: Float>(t31404: F, t7507: F, t7513: F, t174: F, t30779: F, t7322: F, t3125: F, t721: F, t7447: F, t7819: F, t1981: F, t2015: F) -> (F, F, F, F, F) {
    let t31406 = t7507 * t31404 * t7513;
    let t31419 = t7322 * t30779 * t174;
    let t31421 = t31419 * t3125 * t721;
    let t31426 = t7447 * t7819;
    let t31428 = t2015 * t1981;
    (t31406, t31419, t31421, t31426, t31428)
}
