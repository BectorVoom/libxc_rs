//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 222/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk222(t522: f64, t588: f64, t592: f64, t521: f64, t750: f64, t17: f64, t67: f64, t758: f64, t172: f64, t763: f64, t532: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1274 = 4.0_f64 * t588 * t522;
    let t1276 = 4.0_f64 * t592 * t522;
    let t1287 = t521 * t750;
    let t1288 = t17 * t1287;
    let t1291 = t521 * t67;
    let t1293 = 0.18311447306006545054e-3_f64 * t1291 * t758;
    let t1294 = t521 * t172;
    let t1296 = 0.5848223622634646207e0_f64 * t1294 * t763;
    let t1297 = t532 * t571;
    (t1274, t1276, t1287, t1288, t1291, t1293, t1294, t1296, t1297)
}
