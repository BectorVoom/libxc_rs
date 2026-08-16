//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1313/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1313<F: Float>(t1092: F, t27788: F, t92701: F, t1749: F, t303: F, t3191: F, t26692: F, t27808: F, t27904: F, t27911: F, t8038: F, t93366: F, t93394: F, t93590: F, t96138: F, t96141: F, t96148: F, t96150: F, t96154: F) -> (F, F, F) {
    let t96157 = t1092 * t92701 * t27788;
    let t96160 = t303 * t1749 * t3191;
    let t96166 = -t96138 - t93590 - F::cast_from(0.16581944444444444444e-2_f64) * t96141 + F::cast_from(0.12356481481481481482e-2_f64) * t93394 * t8038 + F::cast_from(0.24712962962962962964e-2_f64) * t26692 * t27904 - t96148 + F::cast_from(0.10811921296296296297e-2_f64) * t96150 + F::cast_from(0.33163888888888888888e-2_f64) * t96154 + F::cast_from(0.88437037037037037034e-2_f64) * t96157 + F::cast_from(0.16581944444444444444e-2_f64) * t96160 - F::cast_from(0.18550940104166666667e-3_f64) * t93366 * t27911 - F::cast_from(0.556528203125e-3_f64) * t93366 * t27808;
    (t96157, t96160, t96166)
}
