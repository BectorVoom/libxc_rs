//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1319/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1319(t26796: f64, t303: f64, t4773: f64, t1014: f64, t27971: f64, t27974: f64, t7687: f64, t26681: f64, t26692: f64, t26793: f64, t26806: f64, t27832: f64, t27911: f64, t4961: f64, t8030: f64, t93592: f64, t95549: f64, t96238: f64, t96241: f64, t96247: f64, t96251: f64, t96256: f64) -> (f64, f64, f64) {
    let t96259 = t303 * t26796 * t4773;
    let t96261 = t1014 * t27971;
    let t96264 = 0.46336805555555555556e-3_f64 * t7687 * t27974;
    let t96265 = 0.46336805555555555556e-3_f64 * t27832 * t26681 + 0.37069444444444444444e-2_f64 * t26692 * t27911 + t96238 - 0.13901041666666666667e-2_f64 * t8030 * t26793 - 0.92673611111111111113e-3_f64 * t93592 * t96241 * t4961 * t26806 - 0.22109259259259259258e-2_f64 * t96247 - 0.73697530864197530861e-3_f64 * t96251 - 0.46336805555555555556e-3_f64 * t93592 * t95549 + 0.11054629629629629629e-2_f64 * t96256 - 0.49745833333333333332e-2_f64 * t96259 + 0.88437037037037037034e-2_f64 * t96261 + t96264;
    (t96259, t96261, t96265)
}
