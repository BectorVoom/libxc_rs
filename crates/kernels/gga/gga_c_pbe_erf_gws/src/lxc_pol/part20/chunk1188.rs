//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1188/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1188<F: Float>(t14657: F, t8695: F, t8790: F, t13776: F, t28657: F, t3808: F, t3975: F, t44201: F, t1113: F, t13781: F, t3747: F, t3972: F, t938: F, t15292: F, t840: F, t361: F, t57321: F) -> (F, F, F, F, F, F, F) {
    let t57402 = t14657 * t8695;
    let t57404 = t14657 * t8790;
    let t57410 = t13776 * t3975 * t3808 * t28657;
    let t57415 = t13776 * t3975 * t44201;
    let t57422 = t3972 * t13781 * t1113 * t3747 * t938;
    let t57428 = t840 * t15292;
    let t57432 = t361 * t57321;
    (t57402, t57404, t57410, t57415, t57422, t57428, t57432)
}
