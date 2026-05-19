//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 877/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk877<F: Float>(t16503: F, t16504: F, t571: F, t8425: F, t1598: F, t9163: F, t1550: F, t1624: F, t1627: F, t2024: F, t2402: F, t44700: F, t44702: F, t44705: F, t44713: F, t44724: F, t44727: F, t44734: F, t44738: F, t44740: F, t44744: F, t6415: F, t6418: F, t665: F, t739: F, t8377: F, t8800: F, t903: F) -> F {
    let t44748 = t16503 * t16504 * t571 * t8425;
    let t44752 = t16503 * t16504 * t1598 * t9163;
    let t44754 = F::cast_from(0.1064114997332445985e-4_f64) * t44700 - F::cast_from(0.81823984962736025184e-1_f64) * t44702 + F::cast_from(0.13637330827122670864e0_f64) * t44705 - F::cast_from(0.11974241701863808564e0_f64) * t1550 * t665 * t6415 + F::cast_from(0.17961362552795712846e0_f64) * t903 * t665 * t6418 + F::cast_from(0.11974241701863808564e0_f64) * t739 * t2024 * t44713 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t2402 * t1627 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t8800 * t8377 + F::cast_from(0.8980681276397856423e-1_f64) * t44724 + F::cast_from(0.2993560425465952141e-1_f64) * t44727 - F::cast_from(0.23948483403727617128e0_f64) * t1550 * t2402 * t1624 + F::cast_from(0.21819729323396273382e0_f64) * t44734 + F::cast_from(0.54549323308490683456e-1_f64) * t44738 - F::cast_from(0.74488049813271218945e-4_f64) * t44740 - F::cast_from(0.85129199786595678796e-5_f64) * t44744 + F::cast_from(0.25538759935978703639e-4_f64) * t44748 + F::cast_from(0.25538759935978703638e-4_f64) * t44752;
    t44754
}
