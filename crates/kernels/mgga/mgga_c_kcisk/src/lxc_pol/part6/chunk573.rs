//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 573/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk573<F: Float>(t1266: F, t1275: F, t7993: F, t4126: F, t7976: F, t4129: F, t1234: F, t1264: F, t2129: F, t2143: F, t361: F, t374: F, t4031: F, t4081: F, t45: F, t6035: F, t6095: F, t7922: F, t7928: F, t7960: F, t7963: F, t7970: F, t7978: F) -> (F, F, F) {
    let t7995 = t1266 * t7993 * t1275;
    let t7998 = t4126 * t7976;
    let t7999 = t7998 * t4129;
    let t8002 = -F::new(0.62182e-1) * t7922 * t361 + F::new(2.0) * t6035 * t2129 - F::new(2.0) * t4031 * t7928 + F::new(1.0) * t1234 * t7960 + F::new(0.16081824322151104822e2) * t4081 * t7963 + F::new(0.19751789702565206229e-1) * t45 * t7970 * t374 - F::new(0.11696446794910408142e1) * t6095 * t2143 + F::new(0.11696446794910408142e1) * t1264 * t7978 - F::new(0.58482233974552040708e0) * t1264 * t7995 - F::new(0.17315755899375863299e2) * t1264 * t7999;
    (t7995, t7999, t8002)
}
