//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1017/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1017<F: Float>(t6617: F, t2142: F, t3113: F, t6624: F, t1136: F, t6228: F, t3028: F, t817: F, t1109: F, t2106: F, t1076: F, t2108: F) -> (F, F, F, F, F, F, F) {
    let t9140 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t6617;
    let t9142 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3113 * t2142;
    let t9143 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6624;
    let t9144 = t6228 * t1136;
    let t9145 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t9144;
    let t9147 = t3028 * t817;
    let t9150 = t1109 * t2106;
    let t9159 = t1076 * t2108;
    (t9140, t9142, t9143, t9145, t9147, t9150, t9159)
}
