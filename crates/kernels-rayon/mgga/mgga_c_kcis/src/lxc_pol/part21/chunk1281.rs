//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1281/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1281(t27815: f64, t7703: f64, t9938: f64, t14443: f64, t27821: f64, t1646: f64, t2809: f64, t4947: f64, t93346: f64, t1092: f64, t14649: f64, t3190: f64, t7718: f64) -> (f64, f64, f64, f64, f64) {
    let t95605 = 0.15445601851851851852e-3_f64 * t7703 * t9938 * t27815;
    let t95606 = t14443 * t27821;
    let t95608 = 0.15445601851851851852e-3_f64 * t7703 * t95606;
    let t95621 = t4947 * t93346 * t1646 * t2809;
    let t95626 = t1092 * t7718 * t14649 * t3190;
    (t95605, t95606, t95608, t95621, t95626)
}
