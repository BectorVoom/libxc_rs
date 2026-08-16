//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 938/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk938(t10982: f64, t10989: f64, t11049: f64, t11002: f64, t3857: f64, t895: f64, t1441: f64, t2618: f64, t2593: f64, t1429: f64, t2549: f64, t2621: f64, t3882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11309 = 0.34431666666666666666e0_f64 * t10982;
    let t11312 = 0.13892666666666666667e0_f64 * t10989;
    let t11319 = 0.27785333333333333334e0_f64 * t11049;
    let t11328 = 0.22954444444444444444e0_f64 * t11002;
    let t11351 = t3857 * t895;
    let t11356 = t1441 * t2618;
    let t11362 = t1441 * t2593;
    let t11366 = t1429 * t2549;
    let t11399 = t3882 * t2621;
    (t11309, t11312, t11319, t11328, t11351, t11356, t11362, t11366, t11399)
}
