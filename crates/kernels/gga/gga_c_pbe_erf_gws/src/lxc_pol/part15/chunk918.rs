//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 918/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk918<F: Float>(t6617: F, t2142: F, t3113: F, t6624: F, t1136: F, t6228: F, t3028: F, t817: F, t1109: F, t2106: F, t1076: F, t2108: F, t2848: F, t745: F, t1452: F, t2102: F, t2107: F, t3030: F, t3033: F, t323: F, t6086: F, t6089: F, t6096: F, t8038: F, t818: F, t9050: F) -> (F, F, F, F, F) {
    let t9140 = 35.0 / 216.0 * t6617;
    let t9142 = 7.0 / 144.0 * t3113 * t2142;
    let t9143 = 7.0 / 288.0 * t6624;
    let t9144 = t6228 * t1136;
    let t9145 = 35.0 / 432.0 * t9144;
    let t9147 = t3028 * t817;
    let t9150 = t1109 * t2106;
    let t9159 = t1076 * t2108;
    let t9162 = t2848 * t745;
    let t9165 = t1076 * t1452;
    let t9169 = -t1076 * t6086 - t1452 * t3030 - 2.0 * t2102 * t2848 + 4.0 * t2107 * t9162 + 2.0 * t2107 * t9165 + 2.0 * t2108 * t9150 + 4.0 * t3033 * t6089 + t323 * t9050 - 6.0 * t6096 * t9159 - 2.0 * t745 * t9147 - t8038 * t818;
    (t9140, t9142, t9143, t9145, t9169)
}
