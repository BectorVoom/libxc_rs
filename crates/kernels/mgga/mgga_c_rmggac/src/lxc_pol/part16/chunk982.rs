//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 982/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk982<F: Float>(t40944: F, t40949: F, t40951: F, t40966: F, t44093: F, t44095: F, t46327: F, t46329: F, t46331: F, t46343: F, t46346: F, t46349: F, t10459: F, t321: F, t305: F, t35877: F, t37439: F, t41021: F, t41029: F, t41033: F, t41037: F, t44110: F, t44114: F, t46359: F, t46361: F, t46370: F, t46386: F) -> (F, F, F) {
    let t49323 = 0.11708147441822390596e1 * t40944 - 0.17562221162733585894e1 * t40949 - 0.58540737209111952978e0 * t40951 - 0.40911992481368012595e0 * t46327 + 0.8182398496273602519e0 * t46329 + 0.13637330827122670865e0 * t46331 - 0.16364796992547205038e0 * t46343 + 0.2727466165424534173e0 * t46346 + 0.10909864661698136692e0 * t46349 + 0.72732431077987577948e-1 * t40966 - t44093 + t44095;
    let t49327 = t10459 * t321;
    let t49336 = 0.1333427903096438929e0 * t41021 - 0.40002837092893167871e0 * t41029 + 0.53337116123857557163e0 * t41033 + 0.59871208509319042821e-1 * t305 * t49327 + 0.40911992481368012596e-1 * t46359 + 0.16364796992547205038e0 * t46361 - 0.1454648621559751559e0 * t41037 - t44110 + t44114 - t37439 - 0.20001418546446583936e0 * t35877 - 0.15965655602485078085e0 * t46370 + 0.35922725105591425692e0 * t46386;
    (t49323, t49327, t49336)
}
