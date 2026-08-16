//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 197/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk197(t1284: f64, t76: f64, t16: f64, t239: f64, t252: f64, t954: f64, t957: f64, t960: f64, t240: f64, t20: f64, t259: f64, t253: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1285 = 1.0_f64 / t1284;
    let t1287 = 156.0_f64 * t76 * t1285;
    let t1294 = t239 * t16;
    let t1295 = 1.0_f64 / t1294;
    let t1296 = t252 * t252;
    let t1297 = t1295 * t1296;
    let t1302 = -0.49388888888888888889e-2_f64 * t954 + 0.98777777777777777777e-2_f64 * t957 + 0.13949e-1_f64 * t960;
    let t1303 = t240 * t1302;
    let t1309 = t239 * t239;
    let t1310 = 1.0_f64 / t1309;
    let t1311 = t1310 * t1296;
    let t1314 = t20 * t259;
    let t1315 = t253 * t1314;
    (t1287, t1295, t1296, t1297, t1302, t1303, t1310, t1311, t1315)
}
