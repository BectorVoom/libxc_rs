//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 846/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk846<F: Float>(t38943: F, t8571: F, t39971: F, t39978: F, t39998: F, t40045: F, t45896: F, t45901: F, t45905: F, t45909: F, t45911: F, t45914: F, t45916: F, t45918: F, t45920: F, t45922: F, t45926: F, t45930: F, t45932: F) -> (F,) {
    let t45934 = t8571 * t38943;
    let t45936 = -0.31923449919973379548e-4 * t45896 - 0.51077519871957407276e-4 * t45901 + 0.15323255961587222183e-3 * t45905 - 0.25538759935978703638e-3 * t45909 + t39971 - t39978 + t39998 + 0.1064114997332445985e-4 * t45911 - 0.59590439850616975157e-4 * t40045 + 0.2993560425465952141e-1 * t45914 + 0.72732431077987577941e-1 * t45916 + 0.17961362552795712846e0 * t45918 - 0.35922725105591425692e0 * t45920 - 0.8980681276397856423e-1 * t45922 + 0.25538759935978703638e-4 * t45926 + 0.25538759935978703638e-4 * t45930 - 0.25538759935978703638e-4 * t45932 + 0.10215503974391481455e-3 * t45934;
    (t45936,)
}
