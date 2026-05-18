//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1078/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1078<F: Float>(t31638: F, t31709: F, t1597: F, t1557: F, t19788: F, t26841: F, t26869: F, t30951: F, t30956: F, t30960: F, t30965: F, t30969: F, t30975: F, t30980: F, t30984: F, t30988: F, t30992: F, t31002: F, t31136: F, t31139: F, t31144: F) -> F {
    let t31710 = t31638 + t31709;
    let t31711 = t31710 * t1597;
    let t31719 = -F::new(0.46429444444444444443e-2) * t26841 - F::new(0.11607361111111111111e-2) * t19788 + F::new(0.17411041666666666666e-2) * t30951 + F::new(0.58036805555555555555e-2) * t30956 + F::new(0.69644166666666666666e-2) * t30960 + F::new(0.11607361111111111111e-2) * t30965 - F::new(0.11607361111111111111e-1) * t30969 - F::new(0.34822083333333333333e-2) * t30975 + F::new(0.23214722222222222222e-2) * t30980 - F::new(0.69644166666666666666e-2) * t30984 + F::new(0.58036805555555555556e-2) * t30988 - F::new(0.58036805555555555555e-2) * t30992 - F::new(0.193e0) * t1557 * t31711 - F::new(0.69644166666666666665e-2) * t26869 - F::new(0.46429444444444444443e-2) * t31002 - F::new(0.17411041666666666666e-2) * t31136 - F::new(0.69644166666666666666e-2) * t31139 + F::new(0.10446625e-1) * t31144;
    t31719
}
