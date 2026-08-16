//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2164/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2164(t19815: f64, t6944: f64, t1354: f64, t1827: f64, t91278: f64, t26233: f64, t5289: f64, t22765: f64, t6422: f64, t19921: f64, t6952: f64, t19926: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97246 = t19815 * t6944;
    let t97247 = t97246 * t1354;
    let t97249 = t91278 * t1827;
    let t97251 = t26233 * t5289;
    let t97253 = t22765 * t6422;
    let t97255 = t6952 * t19921;
    let t97257 = t6952 * t19926;
    (t97247, t97249, t97251, t97253, t97255, t97257)
}
