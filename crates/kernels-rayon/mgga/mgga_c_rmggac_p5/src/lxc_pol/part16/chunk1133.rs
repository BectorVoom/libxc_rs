//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1133/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1133(t1737: f64, t698: f64, t25877: f64, t321: f64, t352: f64, t44187: f64, t44194: f64, t46642: f64, t46646: f64, t46648: f64, t46650: f64, t46652: f64, t46656: f64, t46658: f64, t46660: f64, t49445: f64, t5148: f64, t5266: f64, t570: f64) -> (f64, f64) {
    let t49469 = t698 * t1737;
    let t49475 = 0.23948483403727617128e0_f64 * t5266 * t44194 * t570 - 0.11974241701863808564e0_f64 * t5148 * t49445 * t352 + 0.10909864661698136692e0_f64 * t46642 + 0.36366215538993788973e-1_f64 * t46646 + 0.36366215538993788973e0_f64 * t46648 - 0.20455996240684006298e-1_f64 * t46650 + 0.2727466165424534173e-1_f64 * t46652 + 0.68186654135613354325e-2_f64 * t46656 - 0.23948483403727617128e0_f64 * t5148 * t44187 * t570 + 0.71845450211182851384e0_f64 * t25877 * t49469 * t321 + 0.71845450211182851384e0_f64 * t46658 - 0.17961362552795712846e1_f64 * t46660;
    (t49469, t49475)
}
