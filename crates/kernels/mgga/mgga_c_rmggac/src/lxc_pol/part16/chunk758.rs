//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 758/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk758<F: Float>(t2019: F, t2020: F, t8858: F, t8854: F, t8850: F, t22: F, t235: F, t26115: F, t40921: F, t8630: F, t40898: F, t7198: F, t36511: F, t36513: F, t16156: F, t9055: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41604 = t2019 * t2020 * t8858;
    let t41613 = t2019 * t2020 * t8854;
    let t41619 = t2019 * t2020 * t8850;
    let t41634 = t235 * t26115 * t22;
    let t41637 = t8630 * t40921;
    let t41641 = t7198 * t40898;
    let t41647 = 0.19863479950205658386e-3 * t36511;
    let t41648 = 0.19863479950205658386e-3 * t36513;
    let t41654 = t16156 * t9055;
    (t41604, t41613, t41619, t41634, t41637, t41641, t41647, t41648, t41654)
}
