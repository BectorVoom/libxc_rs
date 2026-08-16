//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 911/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk911(t2337: f64, t28: f64, t14: f64, t2341: f64, t8663: f64, t4620: f64, t4714: f64, t8594: f64, t8596: f64, t8598: f64, t8691: f64, t8693: f64, t8695: f64) -> (f64, f64) {
    let t8721 = 1.0_f64 / t2337 / t28;
    let t8722 = t14 * t8721;
    let t8723 = t8663 * t2341;
    let t8725 = 0.96490945932906628932e2_f64 * t8722 * t8723;
    let t8734 = -0.25319e1_f64 * t8594 + 0.16879333333333333333e1_f64 * t8596 - 0.19692555555555555555e1_f64 * t8598 - 0.93011851851851851854e0_f64 * t4620 + 0.13651666666666666667e0_f64 * t8691 - 0.27303333333333333333e0_f64 * t8693 - 0.3185388888888888889e0_f64 * t8695 - 0.36514074074074074075e0_f64 * t4714;
    (t8725, t8734)
}
