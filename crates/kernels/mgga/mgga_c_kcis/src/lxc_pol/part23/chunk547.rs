//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 547/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk547<F: Float>(t1616: F, t4468: F, t1592: F, t1617: F, t4132: F, t4139: F, t4143: F, t4146: F, t4150: F, t4156: F, t4167: F, t4175: F, t4179: F, t4315: F, t4409: F, t4414: F) -> (F, F) {
    let t4469 = t4468 * t1616;
    let t4472 = -F::cast_from(0.23214722222222222222e-2_f64) * t4132 - F::cast_from(0.38691203703703703703e-3_f64) * t4139 + F::cast_from(0.15476481481481481481e-2_f64) * t4143 + F::cast_from(0.23214722222222222222e-2_f64) * t4146 + F::cast_from(0.11607361111111111111e-2_f64) * t4150 + F::cast_from(0.19345601851851851852e-2_f64) * t4156 - F::new(0.13345e0) * t4409 * t1617 + F::cast_from(0.890445125e-2_f64) * t4414 * t4315 - F::cast_from(0.23214722222222222222e-2_f64) * t4167 + F::cast_from(0.15476481481481481481e-2_f64) * t4175 - F::cast_from(0.23214722222222222222e-2_f64) * t4179 - F::new(0.66725e-1) * t1592 * t4469;
    (t4469, t4472)
}
