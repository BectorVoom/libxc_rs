//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 658/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk658(t1118: f64, t3265: f64, t3264: f64, t407: f64, t410: f64, t1102: f64, t3236: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t1100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3266 = t3265 * t1118;
    let t3268 = 2.0_f64 * t3264 * t3266;
    let t3270 = 1.0_f64 / t410 / t407;
    let t3271 = t1102 * t1102;
    let t3272 = t3270 * t3271;
    let t3274 = 4.0_f64 / 9.0_f64 * t3236;
    let t3279 = t3274 - 2.0_f64 / 9.0_f64 * t3238 - 2.0_f64 / 9.0_f64 * t3245 + 2.0_f64 / 3.0_f64 * t3250 + t3254 / 3.0_f64;
    let t3280 = t1100 * t3279;
    (t3266, t3268, t3270, t3271, t3272, t3274, t3279, t3280)
}
