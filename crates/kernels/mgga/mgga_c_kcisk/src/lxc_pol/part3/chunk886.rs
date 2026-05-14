//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 886/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk886<F: Float>(t13429: F, t13433: F, t13454: F, t13459: F, t13466: F, t13470: F, t13940: F, t13947: F, t13952: F, t13956: F, t13960: F, t14613: F, t4347: F, t13962: F, t14154: F, t14157: F, t14160: F, t14162: F, t14167: F, t14171: F, t14173: F, t14177: F, t14179: F, t14181: F, t14185: F) -> (F, F) {
    let t14688 = 0.58036805555555555555e-2 * t13429 + 0.11607361111111111111e-2 * t13433 - 0.223494e0 * t4347 * t14613 + 0.23214722222222222222e-2 * t13454 - 0.69644166666666666666e-2 * t13459 + 0.58036805555555555556e-2 * t13466 - 0.58036805555555555555e-2 * t13470 + 0.18571777777777777778e-1 * t13940 + 0.10446625e-1 * t13947 - 0.34822083333333333333e-2 * t13952 - 0.77382407407407407405e-3 * t13956 + 0.30952962962962962963e-2 * t13960;
    let t14701 = 0.34822083333333333333e-2 * t13962 + 0.17411041666666666666e-2 * t14154 - 0.52233124999999999998e-2 * t14157 - 0.11607361111111111111e-2 * t14160 + 0.34048259259259259259e-1 * t14162 - 0.79445938271604938269e-1 * t14167 + 0.18571777777777777778e-1 * t14171 - 0.18571777777777777778e-1 * t14173 - 0.34822083333333333333e-2 * t14177 - 0.46429444444444444443e-2 * t14179 - 0.34822083333333333333e-2 * t14181 + 0.13928833333333333333e-1 * t14185;
    (t14688, t14701)
}
