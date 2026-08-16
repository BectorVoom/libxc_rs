//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1078/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1078<F: Float>(t31638: F, t31709: F, t1597: F, t1557: F, t19788: F, t26841: F, t26869: F, t30951: F, t30956: F, t30960: F, t30965: F, t30969: F, t30975: F, t30980: F, t30984: F, t30988: F, t30992: F, t31002: F, t31136: F, t31139: F, t31144: F) -> F {
    let t31710 = t31638 + t31709;
    let t31711 = t31710 * t1597;
    let t31719 = -F::cast_from(0.46429444444444444443e-2_f64) * t26841 - F::cast_from(0.11607361111111111111e-2_f64) * t19788 + F::cast_from(0.17411041666666666666e-2_f64) * t30951 + F::cast_from(0.58036805555555555555e-2_f64) * t30956 + F::cast_from(0.69644166666666666666e-2_f64) * t30960 + F::cast_from(0.11607361111111111111e-2_f64) * t30965 - F::cast_from(0.11607361111111111111e-1_f64) * t30969 - F::cast_from(0.34822083333333333333e-2_f64) * t30975 + F::cast_from(0.23214722222222222222e-2_f64) * t30980 - F::cast_from(0.69644166666666666666e-2_f64) * t30984 + F::cast_from(0.58036805555555555556e-2_f64) * t30988 - F::cast_from(0.58036805555555555555e-2_f64) * t30992 - F::cast_from(0.193e0_f64) * t1557 * t31711 - F::cast_from(0.69644166666666666665e-2_f64) * t26869 - F::cast_from(0.46429444444444444443e-2_f64) * t31002 - F::cast_from(0.17411041666666666666e-2_f64) * t31136 - F::cast_from(0.69644166666666666666e-2_f64) * t31139 + F::cast_from(0.10446625e-1_f64) * t31144;
    t31719
}
