//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1158/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1158(t10021: f64, t1336: f64, t1361: f64, t1369: f64, t119: f64, t12286: f64, t12293: f64, t12297: f64, t12361: f64, t1315: f64, t1343: f64, t210: f64, t3733: f64, t3783: f64, t39622: f64, t39892: f64, t40012: f64, t40019: f64, t40022: f64, t40025: f64, t40026: f64, t40035: f64, t40044: f64, t40047: f64, t40052: f64, t40054: f64, t820: f64) -> f64 {
    let t40059 = t1336 * t1361 * t10021;
    let t40060 = t40059 * t1369;
    let t40062 = 7.0_f64 / 36.0_f64 * t40012 - t1315 * t210 * t119 * t39892 / 48.0_f64 + 35.0_f64 / 12.0_f64 * t40019 + 7.0_f64 / 3.0_f64 * t40022 + 5.0_f64 / 4.0_f64 * t40025 * t210 * t119 * t40026 + 3.0_f64 / 16.0_f64 * t3733 * t210 * t119 * t39622 - t40035 * t12293 / 128.0_f64 + t12286 * t12297 / 128.0_f64 + t40044 * t1343 * t820 * t40047 / 128.0_f64 + 35.0_f64 / 96.0_f64 * t40052 + 7.0_f64 / 96.0_f64 * t40054 - t3783 * t12361 / 192.0_f64 + 595.0_f64 / 648.0_f64 * t40060;
    t40062
}
