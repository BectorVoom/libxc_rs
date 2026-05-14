//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1343/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1343<F: Float>(t31032: F, t31280: F, t46089: F, t655: F, t31288: F, t116926: F, t8355: F, t31027: F, t31264: F, t31277: F, t31284: F, t116912: F, t31261: F, t10208: F, t69: F, t96: F) -> (F, F, F, F, F, F, F, F) {
    let t117460 = 50.0 / 27.0 * t31032 * t31280;
    let t117461 = t46089 * t655;
    let t117462 = t117461 * t31288;
    let t117470 = t116926 * t8355;
    let t117473 = 20.0 / 9.0 * t31027 * t31264;
    let t117482 = 20.0 / 9.0 * t31027 * t31277;
    let t117484 = 20.0 / 27.0 * t31032 * t31284;
    let t117497 = 4.0 * t116912 * t31261;
    let t117499 = t69 * t10208 * t96;
    (t117460, t117462, t117470, t117473, t117482, t117484, t117497, t117499)
}
