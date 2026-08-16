//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 390/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk390(t155: f64, t449: f64, t1215: f64, t75: f64, t1216: f64, t456: f64, t1238: f64, t1241: f64, t1243: f64, t1247: f64, t1249: f64, t1251: f64) -> (f64, f64, f64, f64) {
    let t1300 = t155 * t449;
    let t1304 = t75 * t1215;
    let t1305 = t1216 * t456;
    let t1314 = -0.57538888888888888889e0_f64 * t1238 + 0.11507777777777777778e1_f64 * t1241 + 0.40256666666666666667e0_f64 * t1243 + 0.366775e-1_f64 * t1247 + 0.73355e-1_f64 * t1249 + 0.137975e0_f64 * t1251;
    (t1300, t1304, t1305, t1314)
}
