//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1019/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1019<F: Float>(t823: F, t9169: F, t850: F, t852: F, t860: F, t6691: F, t1140: F, t6480: F, t2127: F, t3111: F, t1125: F, t6616: F) -> (F, F, F, F, F, F, F, F) {
    let t9170 = t9169 * t823;
    let t9172 = t850 * t9170 * t852;
    let t9174 = t9172 * t860 / F::cast_from(96.0_f64);
    let t9175 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6691;
    let t9176 = t6480 * t1140;
    let t9177 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t9176;
    let t9179 = t850 * t3111 * t2127;
    let t9181 = t9179 * t860 / F::cast_from(48.0_f64);
    let t9182 = t1125 * t6616;
    (t9170, t9172, t9174, t9175, t9177, t9179, t9181, t9182)
}
