//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1024/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1024(t1737: f64, t352: f64, t2060: f64, t4044: f64, t36978: f64, t6382: f64, t656: f64, t36629: f64, t6387: f64, t36471: f64, t6530: f64, t2604: f64, t9812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46846 = t1737 * t352;
    let t46848 = t4044 * t2060 * t46846;
    let t46853 = t36978 * t656 * t6382;
    let t46856 = t36629 * t656 * t6387;
    let t46859 = t36471 * t656 * t6530;
    let t46861 = t2604 * t9812;
    (t46846, t46848, t46853, t46856, t46859, t46861)
}
