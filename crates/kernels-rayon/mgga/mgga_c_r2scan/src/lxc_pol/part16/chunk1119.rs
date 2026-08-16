//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1119/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1119(t3446: f64, t3447: f64, t40453: f64, t874: f64, t122: f64, t3434: f64, t3437: f64, t3348: f64, t983: f64, t11002: f64, t10831: f64, t1102: f64, t3692: f64) -> (f64, f64, f64, f64) {
    let t40456 = t3446 * t3447 * t40453 * t874;
    let t40457 = 0.30487649791575028314e-3_f64 * t40456;
    let t40460 = t3434 * t3437 * t40453 * t122;
    let t40461 = 0.43368970657079495312e-4_f64 * t40460;
    let t40472 = t3348 * t983;
    let t40473 = t11002 * t40472;
    let t40485 = t1102 * t10831 * t3692;
    (t40457, t40461, t40473, t40485)
}
