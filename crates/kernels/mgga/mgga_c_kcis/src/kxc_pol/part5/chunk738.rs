//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 738/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk738<F: Float>(t1572: F, t6097: F, t2079: F, t4358: F, t1571: F, t1347: F, t1911: F, t1354: F, t2084: F, t1356: F, t5613: F, t1919: F, t3947: F, t1564: F, t1573: F, t1577: F, t1578: F, t2080: F, t4326: F, t4331: F, t4356: F, t4363: F, t4366: F, t4373: F, t5540: F, t5543: F, t5545: F, t5548: F, t5576: F, t5580: F, t5587: F, t601: F, t6072: F, t6075: F, t6080: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6098 = t6097 * t1572;
    let t6101 = t2079 * t4358;
    let t6102 = t6101 * t1571;
    let t6106 = t1911 * t1347;
    let t6111 = t2084 * t1354;
    let t6114 = t5613 * t1356;
    let t6117 = t1919 * t3947;
    let t6118 = t6117 * t1354;
    let t6121 = -0.3109e-1 * t6072 * t601 + 1.0 * t6075 * t1573 + 1.0 * t4326 * t2080 - 2.0 * t4331 * t6080 + 1.0 * t1564 * t6098 + 0.32164683177870697974e2 * t4356 * t6102 + t5540 - t5543 - t5545 + t5548 - t5576 - t5580 - 0.19751789702565206229e-1 * t5587 + 0.58482233974552040708e0 * t6106 * t1578 + 0.58482233974552040708e0 * t4363 * t2084 - 0.11696446794910408142e1 * t4366 * t6111 + 0.58482233974552040708e0 * t1577 * t6114 + 0.17315755899375863299e2 * t4373 * t6118;
    (t6098, t6101, t6102, t6106, t6111, t6114, t6117, t6118, t6121)
}
