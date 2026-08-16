//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 997/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk997<F: Float>(t13429: F, t13433: F, t13454: F, t13459: F, t13466: F, t13470: F, t13940: F, t13947: F, t13952: F, t13956: F, t13960: F, t14613: F, t4347: F) -> F {
    let t14688 = F::cast_from(0.58036805555555555555e-2_f64) * t13429 + F::cast_from(0.11607361111111111111e-2_f64) * t13433 - F::cast_from(0.223494e0_f64) * t4347 * t14613 + F::cast_from(0.23214722222222222222e-2_f64) * t13454 - F::cast_from(0.69644166666666666666e-2_f64) * t13459 + F::cast_from(0.58036805555555555556e-2_f64) * t13466 - F::cast_from(0.58036805555555555555e-2_f64) * t13470 + F::cast_from(0.18571777777777777778e-1_f64) * t13940 + F::cast_from(0.10446625e-1_f64) * t13947 - F::cast_from(0.34822083333333333333e-2_f64) * t13952 - F::cast_from(0.77382407407407407405e-3_f64) * t13956 + F::cast_from(0.30952962962962962963e-2_f64) * t13960;
    t14688
}
