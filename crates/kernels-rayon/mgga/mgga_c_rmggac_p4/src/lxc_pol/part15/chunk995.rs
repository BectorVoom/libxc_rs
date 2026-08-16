//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 995/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk995(t1635: f64, t2347: f64, t27041: f64, t5898: f64, t25820: f64, t2350: f64, t5888: f64, t25854: f64, t5144: f64, t8975: f64, t5267: f64, t45419: f64, t7785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46391 = t2347 * t1635;
    let t46392 = t27041 * t46391;
    let t46394 = t2347 * t5898;
    let t46395 = t25820 * t46394;
    let t46397 = t2350 * t5888;
    let t46398 = t25854 * t46397;
    let t46400 = t8975 * t5144;
    let t46403 = t8975 * t5267;
    let t46406 = t8975 * t5888;
    let t46409 = t7785 * t45419;
    (t46391, t46392, t46394, t46395, t46397, t46398, t46400, t46403, t46406, t46409)
}
