//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 659/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk659<F: Float>(t1572: F, t7459: F, t4358: F, t7443: F, t1356: F, t7002: F, t7019: F, t3947: F, t1564: F, t1577: F, t2080: F, t2084: F, t4331: F, t4356: F, t4366: F, t4373: F, t601: F, t6075: F, t6106: F, t6950: F, t6952: F, t6956: F, t6988: F, t6991: F, t6997: F, t7438: F, t7444: F) -> (F, F, F, F, F, F) {
    let t7460 = t7459 * t1572;
    let t7463 = t7443 * t4358;
    let t7469 = t7002 * t1356;
    let t7472 = t7019 * t1356;
    let t7475 = t7002 * t3947;
    let t7478 = -F::new(0.3109e-1) * t7438 * t601 + F::new(2.0) * t6075 * t2080 - F::new(2.0) * t4331 * t7444 + F::new(1.0) * t1564 * t7460 + F::new(0.32164683177870697974e2) * t4356 * t7463 + t6950 - t6952 + t6956 - t6988 - t6991 - F::new(0.19751789702565206229e-1) * t6997 + F::new(0.11696446794910408142e1) * t6106 * t2084 - F::new(0.11696446794910408142e1) * t4366 * t7469 + F::new(0.58482233974552040708e0) * t1577 * t7472 + F::new(0.17315755899375863299e2) * t4373 * t7475;
    (t7460, t7463, t7469, t7472, t7475, t7478)
}
