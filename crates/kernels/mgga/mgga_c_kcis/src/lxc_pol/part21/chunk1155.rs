//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1155/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1155<F: Float>(t26692: F, t27808: F, t27904: F, t27911: F, t8038: F, t93366: F, t93394: F, t93590: F, t96138: F, t96141: F, t96148: F, t96150: F, t96154: F, t96157: F, t96160: F, t7703: F, t95890: F) -> (F, F) {
    let t96166 = -t96138 - t93590 - 0.16581944444444444444e-2 * t96141 + 0.12356481481481481482e-2 * t93394 * t8038 + 0.24712962962962962964e-2 * t26692 * t27904 - t96148 + 0.10811921296296296297e-2 * t96150 + 0.33163888888888888888e-2 * t96154 + 0.88437037037037037034e-2 * t96157 + 0.16581944444444444444e-2 * t96160 - 0.18550940104166666667e-3 * t93366 * t27911 - 0.556528203125e-3 * t93366 * t27808;
    let t96173 = t7703 * t95890;
    (t96166, t96173)
}
