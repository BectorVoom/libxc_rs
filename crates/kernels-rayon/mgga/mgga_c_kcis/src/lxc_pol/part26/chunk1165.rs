//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1165/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1165(t7492: f64, t7984: f64, t6176: f64, t28395: f64, t28415: f64, t28714: f64, t28727: f64, t28779: f64, t28782: f64, t28784: f64, t28791: f64, t28853: f64, t29305: f64, t29308: f64, t29311: f64, t29510: f64, t7968: f64, t7978: f64, t8213: f64, t8222: f64, t8226: f64) -> (f64, f64, f64) {
    let t29549 = t7984 * t7492;
    let t29550 = t6176 * t29549;
    let t29564 = 0.61782407407407407408e-3_f64 * t28727 * t8222 - 0.24734586805555555556e-3_f64 * t28853 * t8213 + 0.23168402777777777778e-3_f64 * t28779 + 0.23168402777777777778e-3_f64 * t28782 + 0.61782407407407407408e-3_f64 * t28784 + 0.15476481481481481481e-2_f64 * t28395 + 0.34752604166666666667e-3_f64 * t7978 * t29550 - 0.23168402777777777778e-3_f64 * t28791 + 0.46377350260416666667e-4_f64 * t7968 * t29510 + 0.69505208333333333334e-3_f64 * t28714 * t8226 + 0.15476481481481481481e-2_f64 * t28415 - 0.61905925925925925925e-2_f64 * t29305 + 0.11607361111111111111e-2_f64 * t29308 - 0.38691203703703703703e-3_f64 * t29311 + 0.69505208333333333334e-3_f64 * t28714 * t8213;
    (t29549, t29550, t29564)
}
