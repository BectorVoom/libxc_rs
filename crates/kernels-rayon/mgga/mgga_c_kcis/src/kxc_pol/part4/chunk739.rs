//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 739/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk739(t1356: f64, t3938: f64, t3944: f64, t473: f64, t3919: f64, t3947: f64, t1564: f64, t1573: f64, t1577: f64, t1578: f64, t3855: f64, t3858: f64, t3865: f64, t3896: f64, t3904: f64, t3911: f64, t4323: f64, t4326: f64, t4331: f64, t4333: f64, t4351: f64, t4356: f64, t4359: f64, t4363: f64, t4366: f64, t4367: f64, t601: f64) -> (f64, f64, f64, f64) {
    let t4370 = t3938 * t1356;
    let t4373 = t473 * t3944;
    let t4374 = t3919 * t3947;
    let t4377 = -0.3109e-1_f64 * t4323 * t601 + 2.0_f64 * t4326 * t1573 - 2.0_f64 * t4331 * t4333 + 1.0_f64 * t1564 * t4351 + 0.32164683177870697974e2_f64 * t4356 * t4359 + t3855 - t3858 + t3865 - t3896 - t3904 - 0.19751789702565206229e-1_f64 * t3911 + 0.11696446794910408142e1_f64 * t4363 * t1578 - 0.11696446794910408142e1_f64 * t4366 * t4367 + 0.58482233974552040708e0_f64 * t1577 * t4370 + 0.17315755899375863299e2_f64 * t4373 * t4374;
    (t4370, t4373, t4374, t4377)
}
