//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 767/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk767<F: Float>(t9185: F, t9191: F, t9195: F, t9199: F, t9202: F, t9207: F, t9214: F, t9227: F, t9232: F, t9234: F, t9236: F, t9238: F, t38382: F, t38965: F, t39122: F, t39308: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42345 = 0.25538759935978703638e-4 * t9185;
    let t42346 = 0.51077519871957407276e-4 * t9191;
    let t42347 = 0.76616279807936110914e-4 * t9195;
    let t42348 = 0.25538759935978703638e-4 * t9199;
    let t42349 = 0.25538759935978703638e-4 * t9202;
    let t42350 = 0.31923449919973379548e-4 * t9207;
    let t42351 = 0.17025839957319135759e-4 * t9214;
    let t42355 = 0.4726e1 * t9227;
    let t42356 = 0.4726e1 * t9232;
    let t42357 = 0.4726e1 * t9234;
    let t42358 = 0.85129199786595678796e-5 * t9236;
    let t42359 = 0.11974241701863808564e0 * t9238;
    let t42600 = 0.2927036860455597649e0 * t38382;
    let t42793 = 0.66211599834018861287e-4 * t38965;
    let t42856 = 0.66211599834018861287e-4 * t39122;
    let t42913 = 0.66211599834018861287e-4 * t39308;
    (t42345, t42346, t42347, t42348, t42349, t42350, t42351, t42355, t42356, t42357, t42358, t42359, t42600, t42793, t42856, t42913)
}
