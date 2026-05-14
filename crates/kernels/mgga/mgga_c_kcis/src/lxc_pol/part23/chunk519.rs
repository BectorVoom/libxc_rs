//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 519/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk519<F: Float>(t1572: F, t4350: F, t1562: F, t592: F, t600: F, t4332: F, t1341: F, t1347: F, t3918: F, t473: F, t1356: F, t3919: F, t3938: F, t3944: F, t3947: F, t1564: F, t1573: F, t1577: F, t1578: F, t3855: F, t3858: F, t3865: F, t3896: F, t3904: F, t3911: F, t4323: F, t4326: F, t4331: F, t4333: F, t601: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4351 = t4350 * t1572;
    let t4354 = t1562 * t1562;
    let t4355 = 1.0 / t4354;
    let t4356 = t592 * t4355;
    let t4357 = t600 * t600;
    let t4358 = 1.0 / t4357;
    let t4359 = t4332 * t4358;
    let t4363 = t1341 * t1347;
    let t4366 = t473 * t3918;
    let t4367 = t3919 * t1356;
    let t4370 = t3938 * t1356;
    let t4373 = t473 * t3944;
    let t4374 = t3919 * t3947;
    let t4377 = -0.3109e-1 * t4323 * t601 + 2.0 * t4326 * t1573 - 2.0 * t4331 * t4333 + 1.0 * t1564 * t4351 + 0.32164683177870697974e2 * t4356 * t4359 + t3855 - t3858 + t3865 - t3896 - t3904 - 0.19751789702565206229e-1 * t3911 + 0.11696446794910408142e1 * t4363 * t1578 - 0.11696446794910408142e1 * t4366 * t4367 + 0.58482233974552040708e0 * t1577 * t4370 + 0.17315755899375863299e2 * t4373 * t4374;
    (t4351, t4354, t4355, t4356, t4357, t4358, t4359, t4363, t4366, t4367, t4370, t4373, t4374, t4377)
}
