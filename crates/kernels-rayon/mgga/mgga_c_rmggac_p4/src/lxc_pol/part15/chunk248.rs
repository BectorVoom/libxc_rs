//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 248/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk248(t1302: f64, t240: f64, t239: f64, t1296: f64, t20: f64, t259: f64, t253: f64, t40: f64, t41: f64, t21: f64, t22: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1303 = t240 * t1302;
    let t1309 = t239 * t239;
    let t1310 = 1.0_f64 / t1309;
    let t1311 = t1310 * t1296;
    let t1314 = t20 * t259;
    let t1315 = t253 * t1314;
    let t1318 = t40 * t40;
    let t1320 = 1.0_f64 / t41 / t1318;
    let t1321 = t21 * t1320;
    let t1322 = t22 * t22;
    let t1323 = t1321 * t1322;
    (t1303, t1309, t1310, t1311, t1314, t1315, t1318, t1320, t1321, t1322, t1323)
}
