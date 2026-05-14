//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 970/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk970<F: Float>(t4100: F, t7993: F, t6101: F, t13561: F, t7976: F, t1273: F, t13565: F, t1234: F, t1255: F, t1264: F, t13583: F, t13705: F, t20557: F, t20562: F, t2129: F, t2143: F, t26095: F, t26222: F, t26241: F, t26291: F, t26303: F, t26321: F, t361: F, t374: F, t4026: F, t4096: F, t45: F, t6035: F, t6079: F, t6095: F, t6121: F, t7928: F, t7960: F, t7963: F, t7978: F, t7999: F) -> (F,) {
    let t26324 = t4100 * t7993;
    let t26325 = t26324 * t6101;
    let t26328 = t13561 * t7976;
    let t26329 = t13565 * t1273;
    let t26330 = t26328 * t26329;
    let t26333 = 1.0 * t26095 * t1255 + 2.0 * t20562 * t2129 + 2.0 * t6035 * t6079 - 2.0 * t13705 * t7928 + 1.0 * t4026 * t7960 + 1.0 * t1234 * t26222 + 0.16081824322151104822e2 * t13583 * t7963 + 0.19751789702565206229e-1 * t45 * t26241 * t374 - 0.58482233974552040708e0 * t1264 * t26291 + 0.11696446794910408142e1 * t4096 * t7978 - 0.17315755899375863299e2 * t4096 * t7999 - 0.11696446794910408142e1 * t20557 * t2143 - 0.11696446794910408142e1 * t6095 * t6121 - 0.17315755899375863299e2 * t1264 * t26303 - 0.62182e-1 * t26321 * t361 + 0.11696446794910408142e1 * t1264 * t26325 - 0.1025389702100779493e4 * t1264 * t26330;
    (t26333,)
}
