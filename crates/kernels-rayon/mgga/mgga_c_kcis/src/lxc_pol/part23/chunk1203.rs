//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1203/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1203(t1552: f64, t15808: f64, t11776: f64, t2066: f64, t1395: f64, t17433: f64, t17427: f64, t4298: f64, t5748: f64, t6029: f64, t94805: f64, t4303: f64, t5752: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97663 = t15808 * t1552;
    let t97665 = t11776 * t2066;
    let t97667 = t1395 * t17433;
    let t97669 = t1395 * t17427;
    let t97671 = t5748 * t4298;
    let t97673 = t94805 * t6029;
    let t97675 = t5752 * t4303;
    (t97663, t97665, t97667, t97669, t97671, t97673, t97675)
}
