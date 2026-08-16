//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1092/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1092(t2211: f64, t30400: f64, t45916: f64, t45918: f64, t45920: f64, t45922: f64, t45926: f64, t45930: f64, t45932: f64, t45934: f64, t45938: f64, t45942: f64, t45947: f64, t45949: f64, t45951: f64, t45956: f64, t45960: f64, t45964: f64, t739: f64) -> f64 {
    let t48727 = 0.23948483403727617128e0_f64 * t739 * t2211 * t30400 + 0.14546486215597515589e0_f64 * t45916 + 0.35922725105591425692e0_f64 * t45918 - 0.71845450211182851384e0_f64 * t45920 - 0.17961362552795712846e0_f64 * t45922 + 0.5107751987195740728e-4_f64 * t45926 + 0.5107751987195740728e-4_f64 * t45930 - 0.5107751987195740728e-4_f64 * t45932 + 0.20431007948782962912e-3_f64 * t45934 + 0.5107751987195740728e-4_f64 * t45938 - 0.5107751987195740728e-4_f64 * t45942 - 0.47885174879960069325e-4_f64 * t45947 + 0.5107751987195740728e-4_f64 * t45949 - 0.15323255961587222184e-3_f64 * t45951 - 0.5107751987195740728e-4_f64 * t45956 + 0.15323255961587222184e-3_f64 * t45960 - 0.20431007948782962912e-3_f64 * t45964;
    t48727
}
