//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 659/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk659(t1572: f64, t7459: f64, t4358: f64, t7443: f64, t1356: f64, t7002: f64, t7019: f64, t3947: f64, t1564: f64, t1577: f64, t2080: f64, t2084: f64, t4331: f64, t4356: f64, t4366: f64, t4373: f64, t601: f64, t6075: f64, t6106: f64, t6950: f64, t6952: f64, t6956: f64, t6988: f64, t6991: f64, t6997: f64, t7438: f64, t7444: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7460 = t7459 * t1572;
    let t7463 = t7443 * t4358;
    let t7469 = t7002 * t1356;
    let t7472 = t7019 * t1356;
    let t7475 = t7002 * t3947;
    let t7478 = -0.3109e-1_f64 * t7438 * t601 + 2.0_f64 * t6075 * t2080 - 2.0_f64 * t4331 * t7444 + 1.0_f64 * t1564 * t7460 + 0.32164683177870697974e2_f64 * t4356 * t7463 + t6950 - t6952 + t6956 - t6988 - t6991 - 0.19751789702565206229e-1_f64 * t6997 + 0.11696446794910408142e1_f64 * t6106 * t2084 - 0.11696446794910408142e1_f64 * t4366 * t7469 + 0.58482233974552040708e0_f64 * t1577 * t7472 + 0.17315755899375863299e2_f64 * t4373 * t7475;
    (t7460, t7463, t7469, t7472, t7475, t7478)
}
