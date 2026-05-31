//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1490/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1490<F: Float>(t31027: F, t31424: F, t31440: F, t31032: F, t31444: F, t108: F, t1513: F, t116912: F, t31417: F, t31421: F, t2204: F, t5808: F) -> (F, F, F, F, F, F, F) {
    let t117943 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t31027 * t31424;
    let t117976 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31440;
    let t117978 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t31032 * t31444;
    let t117997 = t108 * t1513;
    let t118009 = F::cast_from(4.0_f64) * t116912 * t31417;
    let t118011 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31421;
    let t118089 = F::cast_from(2.0_f64) * t2204 * t5808;
    (t117943, t117976, t117978, t117997, t118009, t118011, t118089)
}
