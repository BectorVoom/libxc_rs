//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1165/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1165<F: Float>(t7492: F, t7984: F, t6176: F, t28395: F, t28415: F, t28714: F, t28727: F, t28779: F, t28782: F, t28784: F, t28791: F, t28853: F, t29305: F, t29308: F, t29311: F, t29510: F, t7968: F, t7978: F, t8213: F, t8222: F, t8226: F) -> (F, F, F) {
    let t29549 = t7984 * t7492;
    let t29550 = t6176 * t29549;
    let t29564 = F::new(0.61782407407407407408e-3) * t28727 * t8222 - F::new(0.24734586805555555556e-3) * t28853 * t8213 + F::new(0.23168402777777777778e-3) * t28779 + F::new(0.23168402777777777778e-3) * t28782 + F::new(0.61782407407407407408e-3) * t28784 + F::new(0.15476481481481481481e-2) * t28395 + F::new(0.34752604166666666667e-3) * t7978 * t29550 - F::new(0.23168402777777777778e-3) * t28791 + F::new(0.46377350260416666667e-4) * t7968 * t29510 + F::new(0.69505208333333333334e-3) * t28714 * t8226 + F::new(0.15476481481481481481e-2) * t28415 - F::new(0.61905925925925925925e-2) * t29305 + F::new(0.11607361111111111111e-2) * t29308 - F::new(0.38691203703703703703e-3) * t29311 + F::new(0.69505208333333333334e-3) * t28714 * t8213;
    (t29549, t29550, t29564)
}
