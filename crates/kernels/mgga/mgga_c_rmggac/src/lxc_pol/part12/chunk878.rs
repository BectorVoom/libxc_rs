//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 878/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk878<F: Float>(t16156: F, t9106: F, t9218: F, t2019: F, t2020: F, t8862: F, t34944: F, t5268: F, t656: F, t236: F, t3351: F, t5207: F, t9188: F) -> (F, F, F, F, F) {
    let t39250 = t16156 * t9106;
    let t39252 = t16156 * t9218;
    let t39255 = t2019 * t2020 * t8862;
    let t39256 = F::cast_from(0.30487649791575028314e-3_f64) * t39255;
    let t39258 = t34944 * t656 * t5268;
    let t39262 = t3351 * t9188 * t236 * t5207;
    (t39250, t39252, t39256, t39258, t39262)
}
