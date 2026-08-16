//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1298/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1298(t1370: f64, t27596: f64, t1307: f64, t28706: f64, t7977: f64, t99247: f64, t27567: f64, t27569: f64, t27583: f64, t27586: f64, t27648: f64, t28714: f64, t98378: f64, t98380: f64, t98396: f64, t99043: f64, t99110: f64, t99201: f64, t99210: f64) -> (f64, f64) {
    let t99320 = t1370 * t27596;
    let t99322 = t99320 * t28706 * t1307;
    let t99331 = t7977 * t99247;
    let t99339 = -0.46336805555555555556e-3_f64 * t27583 * t99201 + 0.30891203703703703704e-3_f64 * t27583 * t99210 - 0.46336805555555555556e-3_f64 * t27583 * t99322 - 0.30918233506944444444e-4_f64 * t27567 * t99043 - 0.23214722222222222222e-2_f64 * t98378 + 0.46429444444444444443e-2_f64 * t98380 + 0.30918233506944444444e-4_f64 * t27567 * t99110 - 0.61782407407407407408e-3_f64 * t99331 * t27586 - 0.61782407407407407408e-3_f64 * t99331 * t27569 - 0.15476481481481481481e-2_f64 * t98396 + 0.34752604166666666667e-3_f64 * t28714 * t27648;
    (t99322, t99339)
}
