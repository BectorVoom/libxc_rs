//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1016/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1016<F: Float>(t41276: F, t8746: F, t41209: F, t8750: F, t41212: F, t41215: F, t7603: F, t41218: F, t41221: F, t41224: F, t41227: F, t8761: F) -> (F, F, F, F, F, F, F, F) {
    let t41277 = t8746 * t41276;
    let t41279 = t8750 * t41209;
    let t41281 = t8750 * t41212;
    let t41283 = t7603 * t41215;
    let t41285 = t7603 * t41218;
    let t41287 = t7603 * t41221;
    let t41289 = t7603 * t41224;
    let t41291 = t8761 * t41227;
    (t41277, t41279, t41281, t41283, t41285, t41287, t41289, t41291)
}
