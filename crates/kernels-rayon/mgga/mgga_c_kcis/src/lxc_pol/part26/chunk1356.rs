//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1356/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1356(t16941: f64, t29361: f64, t7908: f64, t1380: f64, t21827: f64, t27370: f64, t101994: f64, t101997: f64, t102001: f64, t28353: f64, t28369: f64, t28372: f64, t28373: f64, t28420: f64, t5732: f64, t8155: f64, t98025: f64, t98138: f64, t98150: f64, t98155: f64, t98162: f64) -> (f64, f64) {
    let t103219 = t7908 * t16941 * t29361;
    let t103224 = t27370 * t21827 * t1380;
    let t103233 = -0.27802083333333333334e-2_f64 * t7908 * t28372 * t28373 * t5732 - 0.46336805555555555556e-3_f64 * t98025 * t8155 + 0.92673611111111111112e-3_f64 * t28369 * t28420 - 0.10297067901234567901e-3_f64 * t103219 - 0.18550940104166666667e-3_f64 * t98138 + 0.30891203703703703704e-3_f64 * t98150 + 0.13901041666666666667e-2_f64 * t7908 * t103224 - 0.7369753086419753086e-3_f64 * t98162 + 0.88437037037037037034e-2_f64 * t101994 + 0.29479012345679012345e-2_f64 * t101997 + 0.99491666666666666664e-2_f64 * t102001 + 0.14840752083333333333e-2_f64 * t98155 * t28353;
    (t103224, t103233)
}
