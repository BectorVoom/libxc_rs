//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1298/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1298<F: Float>(t1370: F, t27596: F, t1307: F, t28706: F, t7977: F, t99247: F, t27567: F, t27569: F, t27583: F, t27586: F, t27648: F, t28714: F, t98378: F, t98380: F, t98396: F, t99043: F, t99110: F, t99201: F, t99210: F) -> (F, F) {
    let t99320 = t1370 * t27596;
    let t99322 = t99320 * t28706 * t1307;
    let t99331 = t7977 * t99247;
    let t99339 = -F::cast_from(0.46336805555555555556e-3_f64) * t27583 * t99201 + F::cast_from(0.30891203703703703704e-3_f64) * t27583 * t99210 - F::cast_from(0.46336805555555555556e-3_f64) * t27583 * t99322 - F::cast_from(0.30918233506944444444e-4_f64) * t27567 * t99043 - F::cast_from(0.23214722222222222222e-2_f64) * t98378 + F::cast_from(0.46429444444444444443e-2_f64) * t98380 + F::cast_from(0.30918233506944444444e-4_f64) * t27567 * t99110 - F::cast_from(0.61782407407407407408e-3_f64) * t99331 * t27586 - F::cast_from(0.61782407407407407408e-3_f64) * t99331 * t27569 - F::cast_from(0.15476481481481481481e-2_f64) * t98396 + F::cast_from(0.34752604166666666667e-3_f64) * t28714 * t27648;
    (t99322, t99339)
}
