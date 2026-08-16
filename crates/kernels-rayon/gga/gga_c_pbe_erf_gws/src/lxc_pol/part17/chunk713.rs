//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 713/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk713(t1154: f64, t4043: f64, t1158: f64, t4049: f64, t4035: f64, t4047: f64, t4169: f64, t4172: f64, t4174: f64, t4176: f64) -> f64 {
    let t4178 = t4043 * t1154;
    let t4180 = t4049 * t1158;
    let t4182 = t4169 / 96.0_f64 - t4172 / 96.0_f64 - t4035 - t4174 / 48.0_f64 + t4176 / 768.0_f64 - t4178 / 768.0_f64 - t4047 - t4180 / 384.0_f64;
    t4182
}
