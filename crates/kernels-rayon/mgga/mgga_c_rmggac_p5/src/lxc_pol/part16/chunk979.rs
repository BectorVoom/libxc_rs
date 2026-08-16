//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 979/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk979(t1652: f64, t2347: f64, t262: f64, t7788: f64, t45731: f64, t7785: f64, t1734: f64, t2064: f64, t793: f64, t5267: f64, t25820: f64, t5888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46357 = t2347 * t1652;
    let t46358 = t262 * t46357;
    let t46359 = t7788 * t46358;
    let t46361 = t7785 * t45731;
    let t46369 = t2064 * t1734;
    let t46370 = t793 * t46369;
    let t46385 = t2347 * t5267;
    let t46386 = t25820 * t46385;
    let t46388 = t2347 * t5888;
    (t46357, t46358, t46359, t46361, t46369, t46370, t46385, t46386, t46388)
}
