//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 744/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk744<F: Float>(t34761: F, t9165: F, t338: F, t618: F, t16503: F, t35039: F, t7448: F, t9171: F, t34760: F, t8450: F, t7463: F, t38483: F, t38485: F, t38487: F, t38489: F, t38491: F, t38493: F, t38496: F, t38498: F, t38500: F, t38502: F, t38506: F, t38511: F, t38515: F, t38519: F) -> (F, F, F) {
    let t38521 = t34761 * t9165;
    let t38523 = t338 * t618;
    let t38526 = t16503 * t35039 * t38523 * t7448;
    let t38528 = t34761 * t9171;
    let t38530 = t8450 * t34760;
    let t38531 = t38530 * t7463;
    let t38533 = 0.12769379967989351819e-4 * t38483 - 0.25538759935978703638e-4 * t38485 + 0.25538759935978703638e-4 * t38487 + 0.85129199786595678796e-5 * t38489 + 0.25538759935978703638e-4 * t38491 - 0.25538759935978703638e-4 * t38493 + 0.6818665413561335432e-1 * t38496 - 0.85129199786595678796e-5 * t38498 + 0.1064114997332445985e-4 * t38500 + 0.85129199786595678796e-5 * t38502 + 0.85129199786595678796e-5 * t38506 - 0.17025839957319135759e-4 * t38511 - 0.17025839957319135759e-4 * t38515 - 0.1064114997332445985e-4 * t38519 + 0.85129199786595678796e-5 * t38521 - 0.85129199786595678796e-5 * t38526 + 0.85129199786595678796e-5 * t38528 + 0.85129199786595678796e-5 * t38531;
    (t38523, t38530, t38533)
}
