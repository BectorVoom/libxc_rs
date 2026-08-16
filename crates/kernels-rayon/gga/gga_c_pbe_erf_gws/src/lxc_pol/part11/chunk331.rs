//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 331/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk331(t1257: f64, t62: f64, t70: f64, t1231: f64, t31: f64, t4: f64, t542: f64, t155: f64, t388: f64, t174: f64, t405: f64, t27: f64, t387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1258 = 1.0_f64 / t1257;
    let t1259 = t62 * t1258;
    let t1260 = t70 * t70;
    let t1261 = 1.0_f64 / t1260;
    let t1262 = t1231 * t1261;
    let t1266 = t4 * t542 * t31;
    let t1267 = 0.14764770444444444444e-2_f64 * t1266;
    let t1268 = t155 * t388;
    let t1270 = t174 * t1268 * t405;
    let t1271 = 0.35616666666666666667e-1_f64 * t1270;
    let t1272 = t387 * t27;
    (t1258, t1259, t1260, t1261, t1262, t1267, t1268, t1271, t1272)
}
