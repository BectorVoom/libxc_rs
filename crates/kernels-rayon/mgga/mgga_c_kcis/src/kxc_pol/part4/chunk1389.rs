//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1389/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1389(t12119: f64, t12858: f64, t1592: f64, t16656: f64, t16661: f64, t16663: f64, t16668: f64, t16676: f64, t16679: f64, t16688: f64, t16697: f64, t16702: f64, t16704: f64, t16706: f64, t16708: f64, t17731: f64, t17969: f64, t17981: f64, t4409: f64, t4414: f64, t6208: f64) -> f64 {
    let t18034 = -0.11607361111111111111e-2_f64 * t16656 - 0.38691203703703703703e-3_f64 * t16661 - 0.51588271604938271604e-3_f64 * t16663 + 0.69644166666666666664e-2_f64 * t16668 - 0.23214722222222222222e-2_f64 * t16676 - 0.61905925925925925924e-2_f64 * t16679 + 0.13345e0_f64 * t1592 * t17731 + 0.19345601851851851852e-2_f64 * t16688 + 0.51588271604938271605e-2_f64 * t16697 - 0.15476481481481481481e-2_f64 * t12119 - 0.23214722222222222222e-2_f64 * t16702 + 0.890445125e-2_f64 * t4414 * t17969 - 0.178244852896875e-2_f64 * t12858 * t17981 + 0.13345e0_f64 * t4409 * t6208 - 0.13345e0_f64 * t1592 * t17981 - 0.23214722222222222222e-2_f64 * t16704 + 0.15476481481481481481e-2_f64 * t16706 - 0.46429444444444444444e-2_f64 * t16708;
    t18034
}
