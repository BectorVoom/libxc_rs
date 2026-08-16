//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1374/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1374(t11780: f64, t11799: f64, t11811: f64, t11815: f64, t12890: f64, t15800: f64, t15804: f64, t15810: f64, t15813: f64, t15817: f64, t15821: f64, t15824: f64, t15826: f64, t15830: f64, t15832: f64, t15836: f64, t15840: f64, t17731: f64, t4414: f64, t6208: f64) -> f64 {
    let t17738 = 0.38691203703703703703e-3_f64 * t15800 - 0.23214722222222222222e-2_f64 * t15804 + 0.15476481481481481481e-2_f64 * t11780 - 0.23214722222222222222e-2_f64 * t11799 + 0.23214722222222222222e-2_f64 * t15810 - 0.17411041666666666666e-2_f64 * t15813 + 0.46429444444444444443e-2_f64 * t15817 - 0.92858888888888888886e-2_f64 * t15821 + 0.92858888888888888886e-2_f64 * t15824 - 0.25794135802469135802e-3_f64 * t15826 + 0.77382407407407407407e-3_f64 * t15830 - 0.23214722222222222222e-2_f64 * t15832 + 0.12897067901234567901e-2_f64 * t15836 - 0.11607361111111111111e-2_f64 * t15840 + 0.178089025e-1_f64 * t4414 * t17731 + 0.178089025e-1_f64 * t12890 * t6208 + 0.77382407407407407407e-3_f64 * t11811 + 0.12897067901234567901e-2_f64 * t11815;
    t17738
}
