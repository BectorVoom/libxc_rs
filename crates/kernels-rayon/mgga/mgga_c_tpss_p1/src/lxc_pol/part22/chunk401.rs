//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 401/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk401(t1288: f64, t100: f64, t55: f64, t108: f64, t105: f64, t109: f64, t97: f64, tau1: f64) -> (f64, f64, f64, f64, f64) {
    let t1324 = t1288 / 2.0_f64;
    let t1325 = t100 * t1324;
    let t1327 = tau1 * t55;
    let t1329 = -t1324;
    let t1330 = t108 * t1329;
    let t1333 = 5.0_f64 / 3.0_f64 * t105 * t1330 - 5.0_f64 / 3.0_f64 * t1327 * t109 + 5.0_f64 / 3.0_f64 * t97 * t1325;
    (t1324, t1325, t1327, t1329, t1333)
}
