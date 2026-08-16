//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 960/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk960(t1971: f64, t2144: f64, t30283: f64, t3351: f64, t30360: f64, t2289: f64, t38351: f64, t38943: f64, t8571: f64, t39971: f64, t39978: f64, t39998: f64, t40045: f64, t45896: f64, t45901: f64, t45905: f64, t45909: f64, t45911: f64, t45914: f64, t45916: f64, t45918: f64, t45920: f64, t45922: f64) -> f64 {
    let t45926 = t3351 * t1971 * t2144 * t30283;
    let t45930 = t3351 * t1971 * t2144 * t30360;
    let t45932 = t38351 * t2289;
    let t45934 = t8571 * t38943;
    let t45936 = -0.31923449919973379548e-4_f64 * t45896 - 0.51077519871957407276e-4_f64 * t45901 + 0.15323255961587222183e-3_f64 * t45905 - 0.25538759935978703638e-3_f64 * t45909 + t39971 - t39978 + t39998 + 0.1064114997332445985e-4_f64 * t45911 - 0.59590439850616975157e-4_f64 * t40045 + 0.2993560425465952141e-1_f64 * t45914 + 0.72732431077987577941e-1_f64 * t45916 + 0.17961362552795712846e0_f64 * t45918 - 0.35922725105591425692e0_f64 * t45920 - 0.8980681276397856423e-1_f64 * t45922 + 0.25538759935978703638e-4_f64 * t45926 + 0.25538759935978703638e-4_f64 * t45930 - 0.25538759935978703638e-4_f64 * t45932 + 0.10215503974391481455e-3_f64 * t45934;
    t45936
}
