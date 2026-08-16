//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 827/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk827(t338: f64, t618: f64, t16503: f64, t35039: f64, t7448: f64, t34761: f64, t9171: f64, t34760: f64, t8450: f64, t7463: f64, t38483: f64, t38485: f64, t38487: f64, t38489: f64, t38491: f64, t38493: f64, t38496: f64, t38498: f64, t38500: f64, t38502: f64, t38506: f64, t38511: f64, t38515: f64, t38519: f64, t38521: f64) -> (f64, f64, f64) {
    let t38523 = t338 * t618;
    let t38526 = t16503 * t35039 * t38523 * t7448;
    let t38528 = t34761 * t9171;
    let t38530 = t8450 * t34760;
    let t38531 = t38530 * t7463;
    let t38533 = 0.12769379967989351819e-4_f64 * t38483 - 0.25538759935978703638e-4_f64 * t38485 + 0.25538759935978703638e-4_f64 * t38487 + 0.85129199786595678796e-5_f64 * t38489 + 0.25538759935978703638e-4_f64 * t38491 - 0.25538759935978703638e-4_f64 * t38493 + 0.6818665413561335432e-1_f64 * t38496 - 0.85129199786595678796e-5_f64 * t38498 + 0.1064114997332445985e-4_f64 * t38500 + 0.85129199786595678796e-5_f64 * t38502 + 0.85129199786595678796e-5_f64 * t38506 - 0.17025839957319135759e-4_f64 * t38511 - 0.17025839957319135759e-4_f64 * t38515 - 0.1064114997332445985e-4_f64 * t38519 + 0.85129199786595678796e-5_f64 * t38521 - 0.85129199786595678796e-5_f64 * t38526 + 0.85129199786595678796e-5_f64 * t38528 + 0.85129199786595678796e-5_f64 * t38531;
    (t38523, t38530, t38533)
}
