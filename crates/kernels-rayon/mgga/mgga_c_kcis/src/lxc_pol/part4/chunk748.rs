//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 748/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk748(t1616: f64, t4468: f64, t1592: f64, t1617: f64, t4132: f64, t4139: f64, t4143: f64, t4146: f64, t4150: f64, t4156: f64, t4167: f64, t4175: f64, t4179: f64, t4315: f64, t4409: f64, t4414: f64) -> (f64, f64) {
    let t4469 = t4468 * t1616;
    let t4472 = -0.23214722222222222222e-2_f64 * t4132 - 0.38691203703703703703e-3_f64 * t4139 + 0.15476481481481481481e-2_f64 * t4143 + 0.23214722222222222222e-2_f64 * t4146 + 0.11607361111111111111e-2_f64 * t4150 + 0.19345601851851851852e-2_f64 * t4156 - 0.13345e0_f64 * t4409 * t1617 + 0.890445125e-2_f64 * t4414 * t4315 - 0.23214722222222222222e-2_f64 * t4167 + 0.15476481481481481481e-2_f64 * t4175 - 0.23214722222222222222e-2_f64 * t4179 - 0.66725e-1_f64 * t1592 * t4469;
    (t4469, t4472)
}
