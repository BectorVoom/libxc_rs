//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1175/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1175(t2096: f64, t3524: f64, t108: f64, t555: f64, t22: f64, t3528: f64, t105: f64, t13178: f64, t13181: f64, t13182: f64, t13185: f64, t13188: f64, t13191: f64, t13199: f64, t13202: f64, t13203: f64, t1325: f64, t1327: f64, t2078: f64, t2093: f64, t2097: f64, t3515: f64, t3519: f64, t631: f64, t97: f64) -> f64 {
    let t13206 = t3524 * t2096;
    let t13209 = t108 * t555;
    let t13212 = t3528 * t22;
    let t13215 = 200.0_f64 / 27.0_f64 * t2078 * t1325 - 100.0_f64 / 27.0_f64 * t631 * t3515 - 50.0_f64 / 9.0_f64 * t631 * t3519 - 10.0_f64 / 27.0_f64 * t97 * t13178 + 20.0_f64 / 9.0_f64 * t13181 * t13182 + 10.0_f64 / 9.0_f64 * t97 * t13185 + 5.0_f64 / 3.0_f64 * t97 * t13188 - 5.0_f64 * t97 * t13191 - 50.0_f64 / 27.0_f64 * t1327 * t2093 - 25.0_f64 / 9.0_f64 * t1327 * t2097 - 10.0_f64 / 27.0_f64 * t105 * t13199 - 20.0_f64 / 9.0_f64 * t13202 * t13203 + 10.0_f64 / 9.0_f64 * t105 * t13206 - 5.0_f64 / 3.0_f64 * t105 * t13209 + 5.0_f64 * t105 * t13212;
    t13215
}
