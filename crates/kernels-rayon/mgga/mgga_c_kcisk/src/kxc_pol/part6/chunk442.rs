//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 442/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk442(t264: f64, t1097: f64, t1100: f64, t1099: f64, t281: f64, t259: f64, t1128: f64, t278: f64, t2925: f64, t67: f64, t10: f64, t1102: f64, t119: f64, t142: f64, t260: f64, t261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t265 = t264 < -0.66725e-1_f64;
    let t3368 = t1097 * t1100;
    let t3372 = 1.0_f64 / t1099 / t281;
    let t3373 = t259 * t3372;
    let t3374 = t1128 * t1128;
    let t3375 = t278 * t278;
    let t3376 = 1.0_f64 / t3375;
    let t3377 = t3374 * t3376;
    let t3380 = t67 * t2925;
    let t3391 = piecewise3(t265, 0.0_f64, 10.0_f64 / 9.0_f64 * t260 * t3380 * t10 - 20.0_f64 / 27.0_f64 * t260 * t1102 * t142 + 40.0_f64 / 81.0_f64 * t260 * t261 * t119);
    (t3368, t3372, t3373, t3374, t3375, t3376, t3377, t3380, t3391)
}
