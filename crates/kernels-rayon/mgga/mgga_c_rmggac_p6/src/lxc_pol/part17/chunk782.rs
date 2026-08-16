//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 782/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk782(t8444: f64, t8448: f64, t8452: f64, t8460: f64, t8494: f64, t8498: f64, t8505: f64, t8509: f64, t8513: f64, t8523: f64, t8527: f64, t8529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38218 = 0.85129199786595678796e-5_f64 * t8444;
    let t38219 = 0.85129199786595678796e-5_f64 * t8448;
    let t38220 = 0.85129199786595678796e-5_f64 * t8452;
    let t38221 = 0.39914139006212695214e-1_f64 * t8460;
    let t38234 = 0.85129199786595678796e-5_f64 * t8494;
    let t38235 = 0.85129199786595678796e-5_f64 * t8498;
    let t38236 = 0.25538759935978703638e-4_f64 * t8505;
    let t38237 = 0.76616279807936110914e-4_f64 * t8509;
    let t38238 = 0.85129199786595678796e-5_f64 * t8513;
    let t38239 = 0.20455996240684006296e-1_f64 * t8523;
    let t38240 = 0.20455996240684006296e-1_f64 * t8527;
    let t38242 = 0.27274661654245341728e-1_f64 * t8529;
    (t38218, t38219, t38220, t38221, t38234, t38235, t38236, t38237, t38238, t38239, t38240, t38242)
}
