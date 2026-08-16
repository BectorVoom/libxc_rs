//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1022/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1022(t11286: f64, t871: f64, t1185: f64, t3765: f64, t2197: f64, t1184: f64, t9859: f64, t2240: f64, t1197: f64, t3792: f64, t10012: f64, t1196: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11287 = t11286 * t871;
    let t11290 = t1185 * t3765;
    let t11292 = 6.0_f64 * t2197 * t11290;
    let t11293 = t9859 * t1184;
    let t11295 = 0.48245938496077605201e2_f64 * t2240 * t11293;
    let t11296 = t1197 * t3792;
    let t11299 = t10012 * t1196;
    (t11287, t11290, t11292, t11293, t11295, t11296, t11299)
}
