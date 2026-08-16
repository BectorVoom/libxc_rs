//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1004/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1004(t1424: f64, t48032: f64, t40332: f64, t40336: f64, t1457: f64, t1572: f64, t47026: f64, t1429: f64, t46868: f64, t549: f64, t10532: f64, t10533: f64, t47803: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48034 = 0.39722766613167140743e-1_f64 * t48032 * t1424;
    let t48047 = 0.15337170381568299871e1_f64 * t40332;
    let t48048 = 0.38342925953920749677e0_f64 * t40336;
    let t48050 = t1572 * t1457 * t47026;
    let t48055 = 0.39722766613167140743e-1_f64 * t1429 * t549 * t46868;
    let t48060 = t10532 * t10533 * t47803;
    (t48034, t48047, t48048, t48050, t48055, t48060)
}
