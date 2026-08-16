//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1493/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1493<F: Float>(t31032: F, t31280: F, t46089: F, t655: F, t31288: F, t116926: F, t8355: F, t31027: F, t31264: F, t116938: F, t116957: F, t117450: F, t117457: F, t13509: F, t1504: F, t1513: F, t2: F, t31039: F, t31054: F, t31287: F, t4287: F, t8258: F, t8259: F, t8267: F) -> F {
    let t117460 = F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t31032 * t31280;
    let t117461 = t46089 * t655;
    let t117462 = t117461 * t31288;
    let t117470 = t116926 * t8355;
    let t117473 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31264;
    let t117477 = -t117450 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8258 * t31039 * t4287 + t8258 * t8259 * t13509 / F::cast_from(4.0_f64) - F::cast_from(55.0_f64) / F::cast_from(27.0_f64) * t117457 - t117460 + F::cast_from(125.0_f64) / F::cast_from(72.0_f64) * t117462 - F::cast_from(25.0_f64) / F::cast_from(27.0_f64) * t8267 * t116957 * t1504 + F::cast_from(25.0_f64) / F::cast_from(36.0_f64) * t31287 * t31054 * t2 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t117470 + t117473 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t8258 * t116938 * t1513;
    t117477
}
