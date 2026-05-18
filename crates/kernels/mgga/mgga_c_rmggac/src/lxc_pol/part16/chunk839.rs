//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 839/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk839<F: Float>(t34884: F, t9118: F, t2283: F, t34881: F, t2286: F, t7939: F, t2019: F, t2020: F, t8858: F, t8854: F, t8850: F, t22: F, t235: F, t26115: F) -> (F, F, F, F, F, F, F) {
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    let t41585 = t7939 * t2286;
    let t41604 = t2019 * t2020 * t8858;
    let t41613 = t2019 * t2020 * t8854;
    let t41619 = t2019 * t2020 * t8850;
    let t41634 = t235 * t26115 * t22;
    (t41579, t41581, t41585, t41604, t41613, t41619, t41634)
}
