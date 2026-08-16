//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1003/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1003(t4314: f64, t7497: f64, t1615: f64, t1592: f64, t17739: f64, t20987: f64, t20991: f64, t20996: f64, t21000: f64, t21005: f64, t21009: f64, t21015: f64, t21018: f64, t21023: f64) -> (f64, f64, f64) {
    let t22758 = t7497 * t4314;
    let t22759 = t22758 * t1615;
    let t22765 = 0.69644166666666666666e-2_f64 * t20987 + 0.92858888888888888886e-2_f64 * t20991 + t17739 + 0.46429444444444444444e-2_f64 * t20996 - 0.15476481481481481481e-2_f64 * t21000 + 0.46429444444444444444e-2_f64 * t21005 - 0.38691203703703703703e-2_f64 * t21009 + 0.66725e-1_f64 * t1592 * t22759 - 0.46429444444444444444e-2_f64 * t21015 + 0.11607361111111111111e-2_f64 * t21018 + 0.11607361111111111111e-2_f64 * t21023;
    (t22758, t22759, t22765)
}
