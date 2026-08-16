//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 780/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk780(t1354: f64, t6117: f64, t1564: f64, t1573: f64, t1577: f64, t1578: f64, t2080: f64, t2084: f64, t4326: f64, t4331: f64, t4356: f64, t4363: f64, t4366: f64, t4373: f64, t5540: f64, t5543: f64, t5545: f64, t5548: f64, t5576: f64, t5580: f64, t5587: f64, t601: f64, t6072: f64, t6075: f64, t6080: f64, t6098: f64, t6102: f64, t6106: f64, t6111: f64, t6114: f64) -> (f64, f64) {
    let t6118 = t6117 * t1354;
    let t6121 = -0.3109e-1_f64 * t6072 * t601 + 1.0_f64 * t6075 * t1573 + 1.0_f64 * t4326 * t2080 - 2.0_f64 * t4331 * t6080 + 1.0_f64 * t1564 * t6098 + 0.32164683177870697974e2_f64 * t4356 * t6102 + t5540 - t5543 - t5545 + t5548 - t5576 - t5580 - 0.19751789702565206229e-1_f64 * t5587 + 0.58482233974552040708e0_f64 * t6106 * t1578 + 0.58482233974552040708e0_f64 * t4363 * t2084 - 0.11696446794910408142e1_f64 * t4366 * t6111 + 0.58482233974552040708e0_f64 * t1577 * t6114 + 0.17315755899375863299e2_f64 * t4373 * t6118;
    (t6118, t6121)
}
