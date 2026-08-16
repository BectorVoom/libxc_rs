//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 583/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk583<F: Float>(t7430: F, t7438: F, t7582: F, t7594: F, t7627: F, t7662: F, t7708: F, t2231: F, t290: F, t2232: F, t275: F, t7758: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8092 = F::cast_from(0.39726959900411316772e-4_f64) * t7430;
    let t8094 = F::cast_from(0.11918087970123395032e-3_f64) * t7438;
    let t8125 = F::cast_from(0.29568125932752208315e-3_f64) * t7582;
    let t8129 = F::cast_from(0.22223798384940648817e-1_f64) * t7594;
    let t8143 = F::cast_from(0.97567895348519921633e-1_f64) * t7627;
    let t8156 = F::cast_from(0.12981128458281457309e-2_f64) * t7662;
    let t8173 = F::cast_from(0.3193131120497015617e0_f64) * t7708;
    let t8188 = t290 * t2231;
    let t8191 = t275 * t2232;
    let t8192 = F::cast_from(2.0_f64) * t8191;
    let t8193 = F::cast_from(0.1440846329149835838e-2_f64) * t7758;
    (t8092, t8094, t8125, t8129, t8143, t8156, t8173, t8188, t8192, t8193)
}
