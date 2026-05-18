//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1126/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1126<F: Float>(t10459: F, t321: F, t305: F, t35877: F, t37439: F, t41021: F, t41029: F, t41033: F, t41037: F, t44110: F, t44114: F, t46359: F, t46361: F, t46370: F, t46386: F) -> (F, F) {
    let t49327 = t10459 * t321;
    let t49336 = F::new(0.1333427903096438929e0) * t41021 - F::new(0.40002837092893167871e0) * t41029 + F::new(0.53337116123857557163e0) * t41033 + F::new(0.59871208509319042821e-1) * t305 * t49327 + F::new(0.40911992481368012596e-1) * t46359 + F::new(0.16364796992547205038e0) * t46361 - F::new(0.1454648621559751559e0) * t41037 - t44110 + t44114 - t37439 - F::new(0.20001418546446583936e0) * t35877 - F::new(0.15965655602485078085e0) * t46370 + F::new(0.35922725105591425692e0) * t46386;
    (t49327, t49336)
}
