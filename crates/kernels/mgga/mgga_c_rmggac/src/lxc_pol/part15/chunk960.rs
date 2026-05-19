//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 960/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk960<F: Float>(t1971: F, t2144: F, t30283: F, t3351: F, t30360: F, t2289: F, t38351: F, t38943: F, t8571: F, t39971: F, t39978: F, t39998: F, t40045: F, t45896: F, t45901: F, t45905: F, t45909: F, t45911: F, t45914: F, t45916: F, t45918: F, t45920: F, t45922: F) -> F {
    let t45926 = t3351 * t1971 * t2144 * t30283;
    let t45930 = t3351 * t1971 * t2144 * t30360;
    let t45932 = t38351 * t2289;
    let t45934 = t8571 * t38943;
    let t45936 = -F::cast_from(0.31923449919973379548e-4_f64) * t45896 - F::cast_from(0.51077519871957407276e-4_f64) * t45901 + F::cast_from(0.15323255961587222183e-3_f64) * t45905 - F::cast_from(0.25538759935978703638e-3_f64) * t45909 + t39971 - t39978 + t39998 + F::cast_from(0.1064114997332445985e-4_f64) * t45911 - F::cast_from(0.59590439850616975157e-4_f64) * t40045 + F::cast_from(0.2993560425465952141e-1_f64) * t45914 + F::cast_from(0.72732431077987577941e-1_f64) * t45916 + F::cast_from(0.17961362552795712846e0_f64) * t45918 - F::cast_from(0.35922725105591425692e0_f64) * t45920 - F::cast_from(0.8980681276397856423e-1_f64) * t45922 + F::cast_from(0.25538759935978703638e-4_f64) * t45926 + F::cast_from(0.25538759935978703638e-4_f64) * t45930 - F::cast_from(0.25538759935978703638e-4_f64) * t45932 + F::cast_from(0.10215503974391481455e-3_f64) * t45934;
    t45936
}
