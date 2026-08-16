//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1200/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1200<F: Float>(t188: F, t189: F, t193: F, t46952: F, t13830: F, t541: F, t13822: F, t1641: F, t568: F, t569: F, t574: F, t42259: F, t42263: F, t42265: F, t42267: F, t42269: F, t42272: F, t42275: F, t42278: F) -> F {
    let t48107 = F::cast_from(0.35750489951850426669e0_f64) * t188 * t189 * t46952 * t193;
    let t48109 = F::cast_from(0.23833659967900284446e0_f64) * t13830 * t541;
    let t48111 = F::cast_from(0.23005755572352449806e1_f64) * t1641 * t13822;
    let t48115 = F::cast_from(0.23005755572352449806e1_f64) * t574 * t568 * t569 * t46952;
    let t48118 = t48107 + t48109 - t48111 - t48115 + F::cast_from(0.14896037479937677779e-1_f64) * t42259 - t42263 + t42265 + t42267 + F::cast_from(0.71500979903700853338e0_f64) * t42269 + t42272 + t42275 + t42278;
    t48118
}
