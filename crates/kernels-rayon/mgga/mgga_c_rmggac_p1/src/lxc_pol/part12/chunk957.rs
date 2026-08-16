//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 957/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk957(t35697: f64, t35699: f64, t35703: f64, t40354: f64, t40357: f64, t40360: f64, t40362: f64, t40365: f64, t40367: f64, t40372: f64, t40377: f64, t40379: f64, t40384: f64, t40389: f64, t40391: f64, t40396: f64, t40401: f64, t40403: f64) -> f64 {
    let t40405 = 0.59590439850616975157e-4_f64 * t40354 + t40357 - 0.1064114997332445985e-4_f64 * t40360 - 0.53205749866622299248e-5_f64 * t40362 - 0.42564599893297839398e-5_f64 * t40365 - 0.85129199786595678796e-5_f64 * t40367 - 0.31923449919973379548e-4_f64 * t40372 - 0.15961724959986689774e-4_f64 * t40377 + 0.31923449919973379548e-4_f64 * t40379 + 0.31923449919973379548e-4_f64 * t40384 + 0.15961724959986689774e-4_f64 * t40389 + 0.1064114997332445985e-4_f64 * t40391 + 0.1064114997332445985e-4_f64 * t40396 + 0.53205749866622299248e-5_f64 * t40401 - 0.1064114997332445985e-4_f64 * t40403 - t35697 - t35699 - t35703;
    t40405
}
