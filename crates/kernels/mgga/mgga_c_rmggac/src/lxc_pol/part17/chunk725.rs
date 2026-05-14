//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 725/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk725<F: Float>(t16156: F, t9111: F, t9106: F, t9218: F, t2019: F, t2020: F, t8862: F, t7244: F, t8497: F, t3350: F, t39207: F) -> (F, F, F, F, F, F) {
    let t39233 = t16156 * t9111;
    let t39234 = 0.19863479950205658386e-4 * t39233;
    let t39250 = t16156 * t9106;
    let t39252 = t16156 * t9218;
    let t39255 = t2019 * t2020 * t8862;
    let t39256 = 0.30487649791575028314e-3 * t39255;
    let t39264 = t7244 * t8497;
    let t39265 = 0.19863479950205658386e-4 * t39264;
    let t39277 = t39207 * t3350;
    (t39234, t39250, t39252, t39256, t39265, t39277)
}
