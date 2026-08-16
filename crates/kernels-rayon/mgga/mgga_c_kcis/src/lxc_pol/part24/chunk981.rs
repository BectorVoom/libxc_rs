//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 981/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk981(t10473: f64, t11151: f64, t11209: f64, t14609: f64, t15659: f64, t15662: f64, t15671: f64, t19779: f64, t19783: f64, t19787: f64, t19792: f64, t19800: f64, t19802: f64, t19805: f64, t19809: f64, t19813: f64, t19817: f64, t19819: f64, t20550: f64, t430: f64, t6738: f64) -> f64 {
    let t20706 = 0.19345601851851851852e-2_f64 * t19779 + 0.23214722222222222222e-2_f64 * t19783 - 0.11607361111111111111e-2_f64 * t19787 + 0.890445125e-2_f64 * t11151 * t6738 + 0.34822083333333333332e-2_f64 * t19792 + t15659 + t11209 + 0.10317654320987654321e-2_f64 * t10473 - t15662 - t15671 - 0.51588271604938271603e-3_f64 * t14609 + 0.11607361111111111111e-2_f64 * t19800 - 0.15476481481481481481e-2_f64 * t19802 - 0.41270617283950617283e-2_f64 * t19805 - 0.11607361111111111111e-1_f64 * t19809 + 0.51588271604938271605e-2_f64 * t19813 + 0.77382407407407407408e-2_f64 * t19817 + 0.10317654320987654321e-2_f64 * t19819 + t20550 * t430;
    t20706
}
