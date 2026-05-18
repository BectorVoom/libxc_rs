//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1003/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1003<F: Float>(t4314: F, t7497: F, t1615: F, t1592: F, t17739: F, t20987: F, t20991: F, t20996: F, t21000: F, t21005: F, t21009: F, t21015: F, t21018: F, t21023: F) -> (F, F, F) {
    let t22758 = t7497 * t4314;
    let t22759 = t22758 * t1615;
    let t22765 = F::new(0.69644166666666666666e-2) * t20987 + F::new(0.92858888888888888886e-2) * t20991 + t17739 + F::new(0.46429444444444444444e-2) * t20996 - F::new(0.15476481481481481481e-2) * t21000 + F::new(0.46429444444444444444e-2) * t21005 - F::new(0.38691203703703703703e-2) * t21009 + F::new(0.66725e-1) * t1592 * t22759 - F::new(0.46429444444444444444e-2) * t21015 + F::new(0.11607361111111111111e-2) * t21018 + F::new(0.11607361111111111111e-2) * t21023;
    (t22758, t22759, t22765)
}
