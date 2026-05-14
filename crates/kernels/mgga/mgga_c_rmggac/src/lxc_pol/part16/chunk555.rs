//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 555/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk555<F: Float>(t7627: F, t7662: F, t7708: F, t2231: F, t290: F, t2232: F, t275: F, t7758: F, t7780: F, t1347: F, t703: F, t2244: F, t7908: F, t7910: F, t7818: F, t7820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8143 = 0.97567895348519921633e-1 * t7627;
    let t8156 = 0.12981128458281457309e-2 * t7662;
    let t8173 = 0.3193131120497015617e0 * t7708;
    let t8188 = t290 * t2231;
    let t8191 = t275 * t2232;
    let t8192 = 2.0 * t8191;
    let t8193 = 0.1440846329149835838e-2 * t7758;
    let t8197 = 0.15965655602485078085e0 * t7780;
    let t8201 = t1347 * t703;
    let t8208 = t275 * t2244;
    let t8209 = 2.0 * t8208;
    let t8221 = 0.39726959900411316772e-4 * t7908;
    let t8222 = 0.11918087970123395032e-3 * t7910;
    let t8242 = 0.2927036860455597649e0 * t7818;
    let t8243 = 0.66671395154821946452e-1 * t7820;
    (t8143, t8156, t8173, t8188, t8192, t8193, t8197, t8201, t8209, t8221, t8222, t8242, t8243)
}
