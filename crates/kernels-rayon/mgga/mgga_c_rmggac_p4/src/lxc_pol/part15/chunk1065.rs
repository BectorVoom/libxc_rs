//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1065/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1065(t46454: f64, t7192: f64, t2186: f64, t9932: f64, t3928: f64, t6441: f64, t645: f64, t4044: f64, t6421: f64, t2060: f64, t45622: f64, t903: f64) -> (f64, f64, f64, f64, f64) {
    let t47478 = t7192 * t46454;
    let t47484 = t2186 * t9932;
    let t47487 = t3928 * t645 * t6441;
    let t47490 = t4044 * t645 * t6421;
    let t47493 = t903 * t2060 * t45622;
    (t47478, t47484, t47487, t47490, t47493)
}
