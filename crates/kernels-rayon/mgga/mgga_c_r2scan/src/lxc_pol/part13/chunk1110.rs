//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1110/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1110(t3348: f64, t910: f64, t3270: f64, t10667: f64, t11496: f64, t2262: f64, t3262: f64, t3263: f64, t3618: f64, t792: f64, t11002: f64, t3269: f64) -> (f64, f64, f64) {
    let t39323 = t3348 * t910;
    let t39324 = t3270 * t39323;
    let t39326 = 3.0_f64 / 2.0_f64 * t10667 * t39324;
    let t39327 = t11496 * t2262;
    let t39330 = 3.0_f64 / 4.0_f64 * t3262 * t3263 * t39327;
    let t39331 = t3618 * t792;
    let t39332 = t11002 * t39331;
    let t39334 = 5.0_f64 / 8.0_f64 * t3269 * t39332;
    (t39326, t39330, t39334)
}
