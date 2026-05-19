//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 890/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk890<F: Float>(t1572: F, t7459: F, t4358: F, t7443: F, t1356: F, t7002: F, t7019: F, t3947: F, t1564: F, t1577: F, t2080: F, t2084: F, t4331: F, t4356: F, t4366: F, t4373: F, t601: F, t6075: F, t6106: F, t6950: F, t6952: F, t6956: F, t6988: F, t6991: F, t6997: F, t7438: F, t7444: F) -> (F, F, F, F, F, F) {
    let t7460 = t7459 * t1572;
    let t7463 = t7443 * t4358;
    let t7469 = t7002 * t1356;
    let t7472 = t7019 * t1356;
    let t7475 = t7002 * t3947;
    let t7478 = -F::new(0.3109e-1) * t7438 * t601 + F::new(2.0) * t6075 * t2080 - F::new(2.0) * t4331 * t7444 + F::new(1.0) * t1564 * t7460 + F::cast_from(0.32164683177870697974e2_f64) * t4356 * t7463 + t6950 - t6952 + t6956 - t6988 - t6991 - F::cast_from(0.19751789702565206229e-1_f64) * t6997 + F::cast_from(0.11696446794910408142e1_f64) * t6106 * t2084 - F::cast_from(0.11696446794910408142e1_f64) * t4366 * t7469 + F::cast_from(0.58482233974552040708e0_f64) * t1577 * t7472 + F::cast_from(0.17315755899375863299e2_f64) * t4373 * t7475;
    (t7460, t7463, t7469, t7472, t7475, t7478)
}
