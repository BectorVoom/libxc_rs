//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 346/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk346(t1266: f64, t1275: f64, t2141: f64, t1234: f64, t1264: f64, t2115: f64, t2129: f64, t2133: f64, t361: f64, t374: f64, t45: f64, t67: f64) -> (f64, f64) {
    let t2143 = t1266 * t2141 * t1275;
    let t2146 = -0.62182e-1_f64 * t2115 * t361 + 1.0_f64 * t1234 * t2129 + 0.19751789702565206229e-1_f64 * t45 * t2133 * t374 - 0.58482233974552040708e0_f64 * t1264 * t2143;
    let t2147 = t67 * t2146;
    (t2143, t2147)
}
