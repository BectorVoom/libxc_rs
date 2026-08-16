//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 847/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk847(t13076: f64, t13091: f64, t44: f64, t1709: f64, t9985: f64, t2809: f64, t2861: f64, t5027: f64, t5030: f64, t1094: f64, t4922: f64, t1122: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13093 = (t13076 + t13091) * t44;
    let t13097 = t1709 * t9985;
    let t13098 = t13097 * t2809;
    let t13101 = t2861 * t5027;
    let t13102 = 0.33163888888888888888e-2_f64 * t13101;
    let t13103 = t2861 * t5030;
    let t13105 = t4922 * t1094;
    let t13106 = t13105 * sigma0;
    let t13107 = t13106 * t1122;
    (t13093, t13097, t13098, t13101, t13102, t13103, t13105, t13106, t13107)
}
