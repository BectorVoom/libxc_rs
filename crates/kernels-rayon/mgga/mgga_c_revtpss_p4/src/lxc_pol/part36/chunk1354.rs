//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1354/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1354(t104708: f64, t104905: f64, t112350: f64, t112364: f64, t112380: f64, t112397: f64, t136: f64, t1782: f64, t22699: f64, t24817: f64, t24821: f64, t24827: f64, t24831: f64, t29020: f64, t29089: f64, t343: f64, t464: f64, t6625: f64, t6659: f64, t6663: f64, t6690: f64, t7607: f64) -> f64 {
    let t116214 = -0.68598428988911579154e-2_f64 * t29020 * t6625 + 0.13719685797782315831e-1_f64 * t104708 * t6690 - 0.17149607247227894789e-2_f64 * t112364 - 11.0_f64 / 108.0_f64 * t112350 * t1782 + t7607 * t24831 / 36.0_f64 + t29089 * t6659 / 36.0_f64 + t29089 * t6663 / 18.0_f64 - t7607 * t24817 / 288.0_f64 - t7607 * t24821 / 48.0_f64 - 0.17149607247227894789e-2_f64 * t112380 + t112397 / 216.0_f64 - 77.0_f64 / 162.0_f64 * t22699 * t343 * t136 * t464 - 7.0_f64 / 648.0_f64 * t7607 * t24827 - 0.28582678745379824648e-3_f64 * t104905;
    t116214
}
