//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1195/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1195(t40332: f64, t40336: f64, t1457: f64, t1572: f64, t47026: f64, t1429: f64, t46868: f64, t549: f64, t42077: f64, t42081: f64, t42092: f64, t42099: f64, t42138: f64, t42144: f64, t42146: f64, t42151: f64) -> f64 {
    let t48047 = 0.15337170381568299871e1_f64 * t40332;
    let t48048 = 0.38342925953920749677e0_f64 * t40336;
    let t48050 = t1572 * t1457 * t47026;
    let t48055 = 0.39722766613167140743e-1_f64 * t1429 * t549 * t46868;
    let t48056 = -0.61348681526273199483e1_f64 * t42077 - t42081 + t42092 - t42099 + 0.10725146985555128001e1_f64 * t42138 - t48047 - t48048 + 0.71500979903700853338e0_f64 * t48050 - t42144 - 0.25561950635947166451e0_f64 * t42146 + t48055 - t42151;
    t48056
}
