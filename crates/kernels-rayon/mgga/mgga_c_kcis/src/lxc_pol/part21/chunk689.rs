//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 689/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk689(t2155: f64, t7647: f64, t7639: f64, t7598: f64, t7601: f64, t7605: f64, t7634: f64, t7640: f64, t7643: f64, t2161: f64, t898: f64, t2165: f64, t906: f64) -> (f64, f64, f64) {
    let t7648 = t2155 * t7647;
    let t7650 = t2155 * t7639;
    let t7655 = -0.69505208333333333333e-3_f64 * t7634 + 0.92754700520833333333e-4_f64 * t7640 + 0.16217881944444444444e-2_f64 * t7643 + 0.69505208333333333333e-3_f64 * t7648 + 0.69505208333333333333e-3_f64 * t7650 - 0.92858888888888888886e-2_f64 * t7598 + 0.69644166666666666665e-2_f64 * t7601 - 0.69644166666666666665e-2_f64 * t7605;
    let t7657 = t2161 * t898;
    let t7660 = t2165 * t906;
    (t7655, t7657, t7660)
}
