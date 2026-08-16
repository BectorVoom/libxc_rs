//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 384/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk384(t1238: f64, t1241: f64, t1243: f64, t1247: f64, t1249: f64, t1251: f64, t441: f64, t433: f64, t62: f64, t70: f64, t1231: f64, t31: f64, t4: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1253 = -0.78438333333333333333e0_f64 * t1238 + 0.15687666666666666667e1_f64 * t1241 + 0.68863333333333333333e0_f64 * t1243 + 0.14025833333333333333e0_f64 * t1247 + 0.28051666666666666667e0_f64 * t1249 + 0.17365833333333333333e0_f64 * t1251;
    let t1254 = t1253 * t441;
    let t1257 = t433 * t433;
    let t1258 = 1.0_f64 / t1257;
    let t1259 = t62 * t1258;
    let t1260 = t70 * t70;
    let t1261 = 1.0_f64 / t1260;
    let t1262 = t1231 * t1261;
    let t1266 = t4 * t542 * t31;
    (t1253, t1254, t1257, t1258, t1259, t1260, t1261, t1262, t1266)
}
