//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2357/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2357(t104990: f64, t1459: f64, t1774: f64, t19461: f64, t19534: f64, t2165: f64, t27290: f64, t27293: f64, t27371: f64, t4028: f64, t5457: f64, t652: f64, t672: f64, t7408: f64, t7458: f64, t96238: f64, t96833: f64, t96837: f64, t96839: f64, t96842: f64, t96844: f64, t96846: f64, t97777: f64, t97779: f64, t97783: f64, t97785: f64, t97788: f64) -> f64 {
    let t105024 = -2.0_f64 * t19534 * t2165 * t652 - 2.0_f64 * t104990 * t672 - 4.0_f64 * t1459 * t96238 - 2.0_f64 * t1774 * t27371 - 2.0_f64 * t19461 * t2165 - 4.0_f64 * t27290 * t7458 - 4.0_f64 * t27293 * t4028 - 2.0_f64 * t5457 * t7408 + t96833 - t96837 - t96839 - t96842 - t96844 - t96846 + t97777 - t97779 - t97783 - t97785 - t97788;
    t105024
}
