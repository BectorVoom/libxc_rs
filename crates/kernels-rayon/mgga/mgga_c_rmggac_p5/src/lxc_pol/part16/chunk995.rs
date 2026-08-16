//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 995/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk995(t2134: f64, t27: f64, t5840: f64, t649: f64, t46412: f64, t8630: f64, t46416: f64, t7192: f64, t2333: f64, t39953: f64, t7487: f64, t9720: f64) -> (f64, f64, f64, f64, f64) {
    let t46811 = t2134 * t27 * t649 * t5840;
    let t46815 = t8630 * t46412;
    let t46817 = t7192 * t46416;
    let t46819 = t39953 * t2333;
    let t46821 = t7487 * t9720;
    (t46811, t46815, t46817, t46819, t46821)
}
