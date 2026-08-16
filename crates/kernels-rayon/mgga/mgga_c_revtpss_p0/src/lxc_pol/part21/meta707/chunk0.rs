//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2534/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2534(t10022: f64, t2782: f64, t46422: f64, t10013: f64, t2453: f64, t10142: f64, t136: f64, t2457: f64, t3964: f64, t4066: f64, t10139: f64, t1398: f64, t281: f64, t543: f64, t624: f64) -> (f64, f64, f64, f64, f64) {
    let t46493 = t2782 * t10022 * t46422;
    let t46495 = t2453 * t10013;
    let t46496 = t46495 * t10142;
    let t46500 = t3964 * t4066 * t136 * t2457;
    let t46505 = t10139 * t281 * t624 * t1398 * t543;
    (t46493, t46495, t46496, t46500, t46505)
}
