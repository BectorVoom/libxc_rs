//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1483/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1483<F: Float>(t116929: F, t8358: F, t31032: F, t31280: F, t46089: F, t655: F, t31288: F, t116926: F, t8355: F, t31027: F, t31264: F, t31277: F) -> (F, F, F, F, F, F) {
    let t117457 = t116929 * t8358;
    let t117460 = F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t31032 * t31280;
    let t117461 = t46089 * t655;
    let t117462 = t117461 * t31288;
    let t117470 = t116926 * t8355;
    let t117473 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31264;
    let t117482 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31277;
    (t117457, t117460, t117462, t117470, t117473, t117482)
}
