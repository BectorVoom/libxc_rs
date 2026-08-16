//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1045/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1045(t3416: f64, t6767: f64, t1096: f64, t19327: f64, t11153: f64, t1338: f64, t6755: f64, t19309: f64, t3348: f64, t792: f64, t11002: f64, t113: f64, t3268: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37204 = t6767 * t3416;
    let t37209 = t19327 * t1096;
    let t37218 = t1338 * t11153;
    let t37223 = t6755 * t3416;
    let t37226 = t19309 * t1096;
    let t37256 = t3348 * t792;
    let t37257 = t11002 * t37256;
    let t37271 = t97 * t3268 * t113;
    (t37204, t37209, t37218, t37223, t37226, t37257, t37271)
}
