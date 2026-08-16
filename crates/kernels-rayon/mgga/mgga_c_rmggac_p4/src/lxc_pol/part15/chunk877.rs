//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 877/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk877(t16503: f64, t16504: f64, t571: f64, t8425: f64, t1598: f64, t9163: f64, t1550: f64, t1624: f64, t1627: f64, t2024: f64, t2402: f64, t44700: f64, t44702: f64, t44705: f64, t44713: f64, t44724: f64, t44727: f64, t44734: f64, t44738: f64, t44740: f64, t44744: f64, t6415: f64, t6418: f64, t665: f64, t739: f64, t8377: f64, t8800: f64, t903: f64) -> f64 {
    let t44748 = t16503 * t16504 * t571 * t8425;
    let t44752 = t16503 * t16504 * t1598 * t9163;
    let t44754 = 0.1064114997332445985e-4_f64 * t44700 - 0.81823984962736025184e-1_f64 * t44702 + 0.13637330827122670864e0_f64 * t44705 - 0.11974241701863808564e0_f64 * t1550 * t665 * t6415 + 0.17961362552795712846e0_f64 * t903 * t665 * t6418 + 0.11974241701863808564e0_f64 * t739 * t2024 * t44713 + 0.35922725105591425692e0_f64 * t903 * t2402 * t1627 + 0.23948483403727617128e0_f64 * t739 * t8800 * t8377 + 0.8980681276397856423e-1_f64 * t44724 + 0.2993560425465952141e-1_f64 * t44727 - 0.23948483403727617128e0_f64 * t1550 * t2402 * t1624 + 0.21819729323396273382e0_f64 * t44734 + 0.54549323308490683456e-1_f64 * t44738 - 0.74488049813271218945e-4_f64 * t44740 - 0.85129199786595678796e-5_f64 * t44744 + 0.25538759935978703639e-4_f64 * t44748 + 0.25538759935978703638e-4_f64 * t44752;
    t44754
}
