//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 268/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk268(t1266: f64, t1273: f64, t1275: f64, t1229: f64, t1234: f64, t1255: f64, t1260: f64, t1264: f64, t361: f64, t374: f64, t45: f64, t67: f64) -> (f64, f64) {
    let t1276 = t1266 * t1273 * t1275;
    let t1279 = -0.62182e-1_f64 * t1229 * t361 + 1.0_f64 * t1234 * t1255 + 0.19751789702565206229e-1_f64 * t45 * t1260 * t374 - 0.58482233974552040708e0_f64 * t1264 * t1276;
    let t1280 = t67 * t1279;
    (t1276, t1280)
}
