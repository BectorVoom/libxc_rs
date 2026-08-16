//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1019/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1019(t2848: f64, t745: f64, t1076: f64, t1452: f64, t2102: f64, t2107: f64, t2108: f64, t3030: f64, t3033: f64, t323: f64, t6086: f64, t6089: f64, t6096: f64, t8038: f64, t818: f64, t9050: f64, t9147: f64, t9150: f64, t9159: f64) -> f64 {
    let t9162 = t2848 * t745;
    let t9165 = t1076 * t1452;
    let t9169 = -t1076 * t6086 - t1452 * t3030 - 2.0_f64 * t2102 * t2848 + 4.0_f64 * t2107 * t9162 + 2.0_f64 * t2107 * t9165 + 2.0_f64 * t2108 * t9150 + 4.0_f64 * t3033 * t6089 + t323 * t9050 - 6.0_f64 * t6096 * t9159 - 2.0_f64 * t745 * t9147 - t8038 * t818;
    t9169
}
