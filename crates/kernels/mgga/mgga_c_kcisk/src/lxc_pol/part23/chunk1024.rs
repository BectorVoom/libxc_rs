//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1024/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1024<F: Float>(t4120: F, t4129: F, t6124: F, t2115: F, t4030: F, t45: F, t6091: F, t1233: F, t6032: F, t4080: F, t1234: F, t1255: F, t1264: F, t1276: F, t13512: F, t13574: F, t13680: F, t20361: F, t20365: F, t20471: F, t20523: F, t20542: F, t2129: F, t2143: F, t374: F, t4026: F, t4033: F, t4076: F, t4081: F, t4084: F, t4096: F, t4122: F, t6035: F, t6079: F, t6095: F, t6102: F, t6121: F) -> (F,) {
    let t20548 = t4129 * t4120;
    let t20549 = t6124 * t20548;
    let t20552 = t2115 * t4030;
    let t20557 = t45 * t6091;
    let t20562 = t6032 * t1233;
    let t20567 = t2115 * t4080;
    let t20574 = 0.16081824322151104822e2 * t4081 * t20361 + 0.51725014705706168417e3 * t13680 * t20365 - 0.58482233974552040708e0 * t1264 * t20471 - 0.58482233974552040708e0 * t13512 * t2143 + 1.0 * t1234 * t20523 + 0.19751789702565206229e-1 * t45 * t20542 * t374 + 0.23392893589820816284e1 * t4096 * t6102 - 0.17315755899375863299e2 * t1264 * t20549 - 2.0 * t20552 * t4033 - 0.11696446794910408142e1 * t4096 * t6121 - 0.11696446794910408142e1 * t20557 * t1276 - 0.58482233974552040708e0 * t6095 * t4122 + 2.0 * t20562 * t1255 + 1.0 * t6035 * t4076 + 0.16081824322151104822e2 * t20567 * t4084 + 1.0 * t13574 * t2129 + 2.0 * t4026 * t6079;
    (t20574,)
}
