//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 583/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk583(t288: f64, t910: f64, t2858: f64, t481: f64, t2526: f64, t471: f64, t97: f64, t1356: f64, t1387: f64, t1413: f64, t1418: f64, t2272: f64, t2322: f64, t2451: f64, t2453: f64, t2455: f64, t2458: f64, t2460: f64, t2461: f64, t2465: f64, t2485: f64, t2487: f64, t2488: f64, t2853: f64, t2857: f64, t372: f64) -> (f64, f64) {
    let t2859 = t288 * t910;
    let t2861 = t2858 * t2859 * t481;
    let t2862 = 6.0_f64 * t2861;
    let t2864 = t97 * t471 * t2526;
    let t2865 = 3.0_f64 * t2864;
    let t2866 = -0.2363e1_f64 * t2272 + t2460 + t1356 + t2451 + t372 * t2461 - t2453 - t2455 - t2458 + t2465 - t2853 - t2485 + t2487 + t1387 + t2488 + t1413 + t2322 - t2857 - t2862 - t2865 - t1418;
    (t2859, t2866)
}
