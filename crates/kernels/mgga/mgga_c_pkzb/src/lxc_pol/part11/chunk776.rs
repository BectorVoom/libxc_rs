//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 776/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk776<F: Float>(t3208: F, t926: F, t3206: F, t3188: F, t3185: F, t3224: F, t6475: F, t2380: F, t2428: F, t3278: F, t3258: F, t6514: F, t1227: F, t937: F, t2363: F, t3199: F, t410: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8456 = t926 * t3208;
    let t8458 = 0.28582678745379824648e-3 * t3206 * t8456;
    let t8467 = t926 * t3188;
    let t8469 = 0.57165357490759649296e-3 * t3185 * t8467;
    let t8470 = t6475 * t3224;
    let t8472 = 0.57165357490759649296e-3 * t2380 * t8470;
    let t8500 = t2428 * t3278;
    let t8507 = t6514 * t3258;
    let t8511 = t937 * t1227;
    let t8512 = t2363 * t8511;
    let t8515 = t410 * t3199;
    (t8456, t8458, t8467, t8469, t8470, t8472, t8500, t8507, t8512, t8515)
}
