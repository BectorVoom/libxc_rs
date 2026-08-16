//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1313/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1313(t1092: f64, t27788: f64, t92701: f64, t1749: f64, t303: f64, t3191: f64, t26692: f64, t27808: f64, t27904: f64, t27911: f64, t8038: f64, t93366: f64, t93394: f64, t93590: f64, t96138: f64, t96141: f64, t96148: f64, t96150: f64, t96154: f64) -> (f64, f64, f64) {
    let t96157 = t1092 * t92701 * t27788;
    let t96160 = t303 * t1749 * t3191;
    let t96166 = -t96138 - t93590 - 0.16581944444444444444e-2_f64 * t96141 + 0.12356481481481481482e-2_f64 * t93394 * t8038 + 0.24712962962962962964e-2_f64 * t26692 * t27904 - t96148 + 0.10811921296296296297e-2_f64 * t96150 + 0.33163888888888888888e-2_f64 * t96154 + 0.88437037037037037034e-2_f64 * t96157 + 0.16581944444444444444e-2_f64 * t96160 - 0.18550940104166666667e-3_f64 * t93366 * t27911 - 0.556528203125e-3_f64 * t93366 * t27808;
    (t96157, t96160, t96166)
}
