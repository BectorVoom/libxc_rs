//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1040/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1040(t6557: f64, t7778: f64, t903: f64, t26857: f64, t8410: f64, t6355: f64, t8542: f64, t2283: f64, t38355: f64, t8571: f64, t8582: f64, t10194: f64, t1550: f64, t1668: f64, t2868: f64, t302: f64, t4044: f64, t40563: f64, t40565: f64, t40567: f64, t40579: f64, t46365: f64, t46779: f64, t5048: f64, t6394: f64, t6397: f64, t665: f64, t72: f64, t8384: f64, t8817: f64, t884: f64, t9128: f64, t9840: f64) -> f64 {
    let t47100 = t903 * t7778 * t6557;
    let t47108 = t26857 * t8410;
    let t47110 = t6355 * t8542;
    let t47112 = t38355 * t2283;
    let t47114 = t8571 * t8582;
    let t47116 = -t40563 - t40565 + t40567 + 0.23948483403727617128e0_f64 * t2868 * t8384 - t40579 - 0.71845450211182851384e0_f64 * t4044 * t665 * t6394 + 0.11974241701863808564e1_f64 * t5048 * t665 * t6397 - 0.4726e1_f64 * t1668 * t8817 - 0.11974241701863808564e0_f64 * t1550 * t46779 + 0.23948483403727617128e0_f64 * t47100 + 0.11974241701863808564e0_f64 * t884 * t46365 - 0.11974241701863808564e0_f64 * t9128 * t9840 + t72 * t302 * t10194 + 0.17961362552795712846e0_f64 * t47108 + 0.5987120850931904282e-1_f64 * t47110 - 0.85129199786595678796e-5_f64 * t47112 - 0.85129199786595678796e-5_f64 * t47114;
    t47116
}
