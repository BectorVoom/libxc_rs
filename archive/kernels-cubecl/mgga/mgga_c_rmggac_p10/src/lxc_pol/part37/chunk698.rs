//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 698/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk698<F: Float>(t3102: F, t35206: F, t3851: F, t69239: F, t25518: F, t3068: F, t3826: F, t3839: F, t69211: F, t68741: F, t793: F, t1326: F, t14309: F, t2048: F, t352: F) -> (F, F, F, F, F, F, F) {
    let t69289 = t3102 * t35206;
    let t69294 = t3851 * t69239;
    let t69296 = t25518 * t3068;
    let t69303 = t3826 * t69239;
    let t69313 = t3839 * t69211;
    let t69404 = t793 * t68741;
    let t69417 = t14309 * t1326 * t2048 * t352;
    (t69289, t69294, t69296, t69303, t69313, t69404, t69417)
}
