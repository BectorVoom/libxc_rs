//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 739/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk739<F: Float>(t1356: F, t3938: F, t3944: F, t473: F, t3919: F, t3947: F, t1564: F, t1573: F, t1577: F, t1578: F, t3855: F, t3858: F, t3865: F, t3896: F, t3904: F, t3911: F, t4323: F, t4326: F, t4331: F, t4333: F, t4351: F, t4356: F, t4359: F, t4363: F, t4366: F, t4367: F, t601: F) -> (F, F, F, F) {
    let t4370 = t3938 * t1356;
    let t4373 = t473 * t3944;
    let t4374 = t3919 * t3947;
    let t4377 = -F::cast_from(0.3109e-1_f64) * t4323 * t601 + F::cast_from(2.0_f64) * t4326 * t1573 - F::cast_from(2.0_f64) * t4331 * t4333 + F::cast_from(1.0_f64) * t1564 * t4351 + F::cast_from(0.32164683177870697974e2_f64) * t4356 * t4359 + t3855 - t3858 + t3865 - t3896 - t3904 - F::cast_from(0.19751789702565206229e-1_f64) * t3911 + F::cast_from(0.11696446794910408142e1_f64) * t4363 * t1578 - F::cast_from(0.11696446794910408142e1_f64) * t4366 * t4367 + F::cast_from(0.58482233974552040708e0_f64) * t1577 * t4370 + F::cast_from(0.17315755899375863299e2_f64) * t4373 * t4374;
    (t4370, t4373, t4374, t4377)
}
