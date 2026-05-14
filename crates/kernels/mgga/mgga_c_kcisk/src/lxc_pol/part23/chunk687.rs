//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 687/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk687<F: Float>(t1266: F, t1275: F, t6119: F, t2141: F, t4126: F, t1273: F, t4129: F, t1234: F, t1255: F, t1264: F, t1276: F, t2129: F, t2143: F, t361: F, t374: F, t4026: F, t4031: F, t4081: F, t4096: F, t45: F, t6032: F, t6035: F, t6040: F, t6079: F, t6083: F, t6091: F, t6095: F, t6102: F) -> (F, F, F, F, F) {
    let t6121 = t1266 * t6119 * t1275;
    let t6124 = t4126 * t2141;
    let t6125 = t4129 * t1273;
    let t6126 = t6124 * t6125;
    let t6129 = -0.62182e-1 * t6032 * t361 + 1.0 * t6035 * t1255 + 1.0 * t4026 * t2129 - 2.0 * t4031 * t6040 + 1.0 * t1234 * t6079 + 0.16081824322151104822e2 * t4081 * t6083 + 0.19751789702565206229e-1 * t45 * t6091 * t374 - 0.58482233974552040708e0 * t6095 * t1276 - 0.58482233974552040708e0 * t4096 * t2143 + 0.11696446794910408142e1 * t1264 * t6102 - 0.58482233974552040708e0 * t1264 * t6121 - 0.17315755899375863299e2 * t1264 * t6126;
    (t6121, t6124, t6125, t6126, t6129)
}
