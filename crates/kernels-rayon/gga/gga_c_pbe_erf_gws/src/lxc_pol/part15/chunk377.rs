//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 377/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk377(t1184: f64, t1185: f64, t328: f64, t356: f64, t361: f64, t372: f64) -> (f64, f64) {
    let t1186 = t1184 * t1185;
    let t1189 = t356 * t361 * t328;
    let t1190 = t1189 * t372;
    let t1192 = t1186 / 96.0_f64 + t1190 / 1536.0_f64;
    (t1189, t1192)
}
