//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 573/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk573(t1266: f64, t1275: f64, t7993: f64, t4126: f64, t7976: f64, t4129: f64, t1234: f64, t1264: f64, t2129: f64, t2143: f64, t361: f64, t374: f64, t4031: f64, t4081: f64, t45: f64, t6035: f64, t6095: f64, t7922: f64, t7928: f64, t7960: f64, t7963: f64, t7970: f64, t7978: f64) -> (f64, f64, f64) {
    let t7995 = t1266 * t7993 * t1275;
    let t7998 = t4126 * t7976;
    let t7999 = t7998 * t4129;
    let t8002 = -0.62182e-1_f64 * t7922 * t361 + 2.0_f64 * t6035 * t2129 - 2.0_f64 * t4031 * t7928 + 1.0_f64 * t1234 * t7960 + 0.16081824322151104822e2_f64 * t4081 * t7963 + 0.19751789702565206229e-1_f64 * t45 * t7970 * t374 - 0.11696446794910408142e1_f64 * t6095 * t2143 + 0.11696446794910408142e1_f64 * t1264 * t7978 - 0.58482233974552040708e0_f64 * t1264 * t7995 - 0.17315755899375863299e2_f64 * t1264 * t7999;
    (t7995, t7999, t8002)
}
