//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 512/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk512(t1266: f64, t1275: f64, t4120: f64, t1265: f64, t4101: f64, t373: f64, t1234: f64, t1255: f64, t1264: f64, t1276: f64, t361: f64, t374: f64, t4023: f64, t4026: f64, t4031: f64, t4033: f64, t4076: f64, t4081: f64, t4084: f64, t4092: f64, t4096: f64, t4103: f64, t45: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4122 = t1266 * t4120 * t1275;
    let t4125 = t1265 * t1265;
    let t4126 = 1.0_f64 / t4125;
    let t4127 = t4126 * t4101;
    let t4128 = t373 * t373;
    let t4129 = 1.0_f64 / t4128;
    let t4130 = t4127 * t4129;
    let t4133 = -0.62182e-1_f64 * t4023 * t361 + 2.0_f64 * t4026 * t1255 - 2.0_f64 * t4031 * t4033 + 1.0_f64 * t1234 * t4076 + 0.16081824322151104822e2_f64 * t4081 * t4084 + 0.19751789702565206229e-1_f64 * t45 * t4092 * t374 - 0.11696446794910408142e1_f64 * t4096 * t1276 + 0.11696446794910408142e1_f64 * t1264 * t4103 - 0.58482233974552040708e0_f64 * t1264 * t4122 - 0.17315755899375863299e2_f64 * t1264 * t4130;
    (t4122, t4125, t4126, t4128, t4129, t4130, t4133)
}
