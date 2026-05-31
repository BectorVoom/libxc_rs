//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 590/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk590<F: Float>(t1354: F, t6117: F, t1564: F, t1573: F, t1577: F, t1578: F, t2080: F, t2084: F, t4326: F, t4331: F, t4356: F, t4363: F, t4366: F, t4373: F, t5540: F, t5543: F, t5545: F, t5548: F, t5576: F, t5580: F, t5587: F, t601: F, t6072: F, t6075: F, t6080: F, t6098: F, t6102: F, t6106: F, t6111: F, t6114: F) -> (F, F) {
    let t6118 = t6117 * t1354;
    let t6121 = -F::cast_from(0.3109e-1_f64) * t6072 * t601 + F::cast_from(1.0_f64) * t6075 * t1573 + F::cast_from(1.0_f64) * t4326 * t2080 - F::cast_from(2.0_f64) * t4331 * t6080 + F::cast_from(1.0_f64) * t1564 * t6098 + F::cast_from(0.32164683177870697974e2_f64) * t4356 * t6102 + t5540 - t5543 - t5545 + t5548 - t5576 - t5580 - F::cast_from(0.19751789702565206229e-1_f64) * t5587 + F::cast_from(0.58482233974552040708e0_f64) * t6106 * t1578 + F::cast_from(0.58482233974552040708e0_f64) * t4363 * t2084 - F::cast_from(0.11696446794910408142e1_f64) * t4366 * t6111 + F::cast_from(0.58482233974552040708e0_f64) * t1577 * t6114 + F::cast_from(0.17315755899375863299e2_f64) * t4373 * t6118;
    (t6118, t6121)
}
