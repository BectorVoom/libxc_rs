//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1258/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1258(t98057: f64, t18210: f64, t28737: f64, t7968: f64, t28707: f64, t27595: f64, t27607: f64, t28741: f64, t27563: f64, t28721: f64, t98104: f64, t7978: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t99052 = 0.15476481481481481481e-2_f64 * t98057;
    let t99056 = t18210 * t28737;
    let t99058 = 0.30918233506944444444e-4_f64 * t7968 * t99056;
    let t99059 = t18210 * t28707;
    let t99060 = t27595 * t99059;
    let t99065 = 0.7722800925925925926e-4_f64 * t27607 * t28741;
    let t99069 = t28721 * t27563;
    let t99082 = 0.15476481481481481481e-2_f64 * t98104;
    let t99098 = 0.23168402777777777778e-3_f64 * t7978 * t99056;
    (t99052, t99058, t99059, t99060, t99065, t99069, t99082, t99098)
}
