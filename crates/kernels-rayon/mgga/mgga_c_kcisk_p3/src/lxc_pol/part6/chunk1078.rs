//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1078/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1078(t31638: f64, t31709: f64, t1597: f64, t1557: f64, t19788: f64, t26841: f64, t26869: f64, t30951: f64, t30956: f64, t30960: f64, t30965: f64, t30969: f64, t30975: f64, t30980: f64, t30984: f64, t30988: f64, t30992: f64, t31002: f64, t31136: f64, t31139: f64, t31144: f64) -> f64 {
    let t31710 = t31638 + t31709;
    let t31711 = t31710 * t1597;
    let t31719 = -0.46429444444444444443e-2_f64 * t26841 - 0.11607361111111111111e-2_f64 * t19788 + 0.17411041666666666666e-2_f64 * t30951 + 0.58036805555555555555e-2_f64 * t30956 + 0.69644166666666666666e-2_f64 * t30960 + 0.11607361111111111111e-2_f64 * t30965 - 0.11607361111111111111e-1_f64 * t30969 - 0.34822083333333333333e-2_f64 * t30975 + 0.23214722222222222222e-2_f64 * t30980 - 0.69644166666666666666e-2_f64 * t30984 + 0.58036805555555555556e-2_f64 * t30988 - 0.58036805555555555555e-2_f64 * t30992 - 0.193e0_f64 * t1557 * t31711 - 0.69644166666666666665e-2_f64 * t26869 - 0.46429444444444444443e-2_f64 * t31002 - 0.17411041666666666666e-2_f64 * t31136 - 0.69644166666666666666e-2_f64 * t31139 + 0.10446625e-1_f64 * t31144;
    t31719
}
