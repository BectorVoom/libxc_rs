//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1885/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1885(t22765: f64, t6422: f64, t19921: f64, t6952: f64, t19926: f64, t22756: f64, t22783: f64, t6431: f64, t1831: f64, t91160: f64, t19815: f64, t6951: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97253 = t22765 * t6422;
    let t97255 = t6952 * t19921;
    let t97257 = t6952 * t19926;
    let t97259 = t22756 * t6422;
    let t97261 = t22783 * t6431;
    let t97263 = t91160 * t1831;
    let t97265 = t19815 * t6951;
    (t97253, t97255, t97257, t97259, t97261, t97263, t97265)
}
