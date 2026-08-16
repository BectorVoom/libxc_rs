//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1060/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1060(t1990: f64, t9826: f64, t6355: f64, t9005: f64, t11905: f64, t2301: f64, t10050: f64, t36612: f64, t46867: f64, t739: f64, t7577: f64, t2131: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47405 = t9826 * t1990;
    let t47408 = t6355 * t9005;
    let t47410 = t11905 * t2301;
    let t47414 = t36612 * t10050;
    let t47417 = t739 * t7577 * t46867;
    let t47421 = t6624 * t2131;
    (t47405, t47408, t47410, t47414, t47417, t47421)
}
