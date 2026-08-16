//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1378/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1378(t1571: f64, t17815: f64, t4350: f64, t6101: f64, t12732: f64, t2079: f64, t4332: f64, t12689: f64, t12730: f64, t12761: f64, t12767: f64, t12772: f64, t1577: f64, t17792: f64, t17797: f64, t17806: f64, t17809: f64, t17812: f64, t2080: f64, t4331: f64, t4351: f64, t4356: f64, t4359: f64, t4363: f64, t6075: f64, t6080: f64, t6102: f64, t6114: f64) -> f64 {
    let t17816 = t17815 * t1571;
    let t17819 = t6101 * t4350;
    let t17822 = t2079 * t12732;
    let t17823 = t17822 * t4332;
    let t17826 = 0.11696446794910408142e1_f64 * t4363 * t6114 + 0.58482233974552040708e0_f64 * t1577 * t17792 + 1.0_f64 * t6075 * t4351 + 0.32164683177870697974e2_f64 * t17797 * t4359 + 1.0_f64 * t12767 * t2080 - 4.0_f64 * t12761 * t6080 + 0.64329366355741395948e2_f64 * t12772 * t6102 - 4.0_f64 * t4331 * t17806 - 2.0_f64 * t4331 * t17809 - 0.19298809906722418785e3_f64 * t12689 * t17812 + 0.64329366355741395948e2_f64 * t4356 * t17816 + 0.32164683177870697974e2_f64 * t4356 * t17819 + 0.20691336878655965246e4_f64 * t12730 * t17823;
    t17826
}
