//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1126/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1126(t10459: f64, t321: f64, t305: f64, t35877: f64, t37439: f64, t41021: f64, t41029: f64, t41033: f64, t41037: f64, t44110: f64, t44114: f64, t46359: f64, t46361: f64, t46370: f64, t46386: f64) -> (f64, f64) {
    let t49327 = t10459 * t321;
    let t49336 = 0.1333427903096438929e0_f64 * t41021 - 0.40002837092893167871e0_f64 * t41029 + 0.53337116123857557163e0_f64 * t41033 + 0.59871208509319042821e-1_f64 * t305 * t49327 + 0.40911992481368012596e-1_f64 * t46359 + 0.16364796992547205038e0_f64 * t46361 - 0.1454648621559751559e0_f64 * t41037 - t44110 + t44114 - t37439 - 0.20001418546446583936e0_f64 * t35877 - 0.15965655602485078085e0_f64 * t46370 + 0.35922725105591425692e0_f64 * t46386;
    (t49327, t49336)
}
