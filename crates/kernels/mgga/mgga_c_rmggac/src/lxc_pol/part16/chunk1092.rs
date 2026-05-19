//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1092/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1092<F: Float>(t2211: F, t30400: F, t45916: F, t45918: F, t45920: F, t45922: F, t45926: F, t45930: F, t45932: F, t45934: F, t45938: F, t45942: F, t45947: F, t45949: F, t45951: F, t45956: F, t45960: F, t45964: F, t739: F) -> F {
    let t48727 = F::cast_from(0.23948483403727617128e0_f64) * t739 * t2211 * t30400 + F::cast_from(0.14546486215597515589e0_f64) * t45916 + F::cast_from(0.35922725105591425692e0_f64) * t45918 - F::cast_from(0.71845450211182851384e0_f64) * t45920 - F::cast_from(0.17961362552795712846e0_f64) * t45922 + F::cast_from(0.5107751987195740728e-4_f64) * t45926 + F::cast_from(0.5107751987195740728e-4_f64) * t45930 - F::cast_from(0.5107751987195740728e-4_f64) * t45932 + F::cast_from(0.20431007948782962912e-3_f64) * t45934 + F::cast_from(0.5107751987195740728e-4_f64) * t45938 - F::cast_from(0.5107751987195740728e-4_f64) * t45942 - F::cast_from(0.47885174879960069325e-4_f64) * t45947 + F::cast_from(0.5107751987195740728e-4_f64) * t45949 - F::cast_from(0.15323255961587222184e-3_f64) * t45951 - F::cast_from(0.5107751987195740728e-4_f64) * t45956 + F::cast_from(0.15323255961587222184e-3_f64) * t45960 - F::cast_from(0.20431007948782962912e-3_f64) * t45964;
    t48727
}
