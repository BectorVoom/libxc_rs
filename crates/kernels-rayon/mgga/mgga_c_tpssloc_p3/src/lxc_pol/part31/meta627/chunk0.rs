//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1884/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1884(t1827: f64, t91285: f64, t22756: f64, t6417: f64, t19868: f64, t6945: f64, t19815: f64, t6944: f64, t1354: f64, t91278: f64, t26233: f64, t5289: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97240 = t91285 * t1827;
    let t97242 = t22756 * t6417;
    let t97244 = t6945 * t19868;
    let t97246 = t19815 * t6944;
    let t97247 = t97246 * t1354;
    let t97249 = t91278 * t1827;
    let t97251 = t26233 * t5289;
    (t97240, t97242, t97244, t97247, t97249, t97251)
}
