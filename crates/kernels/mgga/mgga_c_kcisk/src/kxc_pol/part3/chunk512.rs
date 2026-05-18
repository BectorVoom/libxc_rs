//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 512/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk512<F: Float>(t1266: F, t1275: F, t4120: F, t1265: F, t4101: F, t373: F, t1234: F, t1255: F, t1264: F, t1276: F, t361: F, t374: F, t4023: F, t4026: F, t4031: F, t4033: F, t4076: F, t4081: F, t4084: F, t4092: F, t4096: F, t4103: F, t45: F) -> (F, F, F, F, F, F, F) {
    let t4122 = t1266 * t4120 * t1275;
    let t4125 = t1265 * t1265;
    let t4126 = F::new(1.0) / t4125;
    let t4127 = t4126 * t4101;
    let t4128 = t373 * t373;
    let t4129 = F::new(1.0) / t4128;
    let t4130 = t4127 * t4129;
    let t4133 = -F::new(0.62182e-1) * t4023 * t361 + F::new(2.0) * t4026 * t1255 - F::new(2.0) * t4031 * t4033 + F::new(1.0) * t1234 * t4076 + F::new(0.16081824322151104822e2) * t4081 * t4084 + F::new(0.19751789702565206229e-1) * t45 * t4092 * t374 - F::new(0.11696446794910408142e1) * t4096 * t1276 + F::new(0.11696446794910408142e1) * t1264 * t4103 - F::new(0.58482233974552040708e0) * t1264 * t4122 - F::new(0.17315755899375863299e2) * t1264 * t4130;
    (t4122, t4125, t4126, t4128, t4129, t4130, t4133)
}
