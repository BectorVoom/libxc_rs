//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 997/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk997(t13429: f64, t13433: f64, t13454: f64, t13459: f64, t13466: f64, t13470: f64, t13940: f64, t13947: f64, t13952: f64, t13956: f64, t13960: f64, t14613: f64, t4347: f64) -> f64 {
    let t14688 = 0.58036805555555555555e-2_f64 * t13429 + 0.11607361111111111111e-2_f64 * t13433 - 0.223494e0_f64 * t4347 * t14613 + 0.23214722222222222222e-2_f64 * t13454 - 0.69644166666666666666e-2_f64 * t13459 + 0.58036805555555555556e-2_f64 * t13466 - 0.58036805555555555555e-2_f64 * t13470 + 0.18571777777777777778e-1_f64 * t13940 + 0.10446625e-1_f64 * t13947 - 0.34822083333333333333e-2_f64 * t13952 - 0.77382407407407407405e-3_f64 * t13956 + 0.30952962962962962963e-2_f64 * t13960;
    t14688
}
