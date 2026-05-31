//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 943/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk943<F: Float>(t339: F, t6094: F, t16463: F, t333: F, t56: F, t338: F, t348: F, t15651: F, t191: F, t22: F, t364: F, t369: F, t371: F) -> (F, F, F, F) {
    let t21623 = t6094 * t339;
    let t21637 = t16463 * t56 * t333;
    let t21640 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t348 * t21637 * t338;
    let t21647 = F::cast_from(13685.0_f64) / F::cast_from(31104.0_f64) * t364 / t22 / t15651 * t191 * t369 * t371;
    (t21623, t21637, t21640, t21647)
}
