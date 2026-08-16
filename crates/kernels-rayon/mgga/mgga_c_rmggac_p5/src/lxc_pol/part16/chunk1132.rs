//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1132/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1132(t1756: f64, t8264: f64, t1734: f64, t698: f64, t118: f64, t1652: f64, t305: f64, t321: f64, t333: f64, t338: f64, t37584: f64, t46612: f64, t46614: f64, t46634: f64, t4669: f64, t48489: f64, t48539: f64, t48591: f64, t48638: f64, t49154: f64, t5259: f64, t5266: f64, t9540: f64) -> (f64, f64, f64) {
    let t49432 = t8264 * t1756;
    let t49445 = t698 * t1734;
    let t49452 = 0.5987120850931904282e-1_f64 * t46612 - 0.47896966807455234256e0_f64 * t46614 + 0.11974241701863808564e0_f64 * t118 * t48539 + t37584 + 0.19957069503106347607e-1_f64 * t118 * t338 * t49154 - 0.39914139006212695214e-1_f64 * t118 * t49432 + 0.31931311204970156171e0_f64 * t46634 + 0.23948483403727617128e0_f64 * t5266 * t9540 * t1652 + 0.59871208509319042821e-1_f64 * t305 * t48591 + 0.59871208509319042821e-1_f64 * t305 * t48489 + 0.11974241701863808564e0_f64 * t305 * t48638 + 0.11974241701863808564e0_f64 * t5259 * t49445 * t321 - 0.17961362552795712846e0_f64 * t4669 * t49445 * t333;
    (t49432, t49445, t49452)
}
