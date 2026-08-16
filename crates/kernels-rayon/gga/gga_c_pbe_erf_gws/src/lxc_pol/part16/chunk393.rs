//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 393/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk393(t1216: f64, t1322: f64, t1224: f64, t1230: f64, t1232: f64, t1254: f64, t1259: f64, t1262: f64, t1267: f64, t1271: f64, t1278: f64, t1288: f64, t1296: f64, t1300: f64, t1304: f64, t1305: f64, t1315: f64, t1320: f64, t174: f64, t4: f64, t435: f64, t442: f64, t450: f64, t457: f64, t542: f64, t71: f64, t84: f64) -> (f64, f64) {
    let t1323 = t1216 * t1322;
    let t1326 = -0.70981924444444444442e-3_f64 * t4 * t542 * t71 - 0.34246666666666666666e-1_f64 * t174 * t1224 * t442 - 2.0_f64 * t1230 * t1232 + 1.0_f64 * t435 * t1254 + 0.32164683177870697974e2_f64 * t1259 * t1262 + t1267 + t1271 + t1278 - t1288 - t1296 - 0.24415406715670879921e-3_f64 * t4 * t542 * t84 - 0.10843580882781524214e-1_f64 * t174 * t1300 * t457 - 0.11696446794910408142e1_f64 * t1304 * t1305 + 0.58482233974552040708e0_f64 * t450 * t1315 + 0.17315755899375863299e2_f64 * t1320 * t1323;
    (t1323, t1326)
}
