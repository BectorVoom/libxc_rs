//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1291/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1291<F: Float>(t13142: F, t13290: F, t2501: F, t2503: F, t28413: F, t28487: F, t326: F, t338: F, t353: F, t36340: F, t376: F, t39490: F, t39510: F, t39521: F, t39523: F, t46892: F, t46914: F, t46928: F, t48985: F, t49464: F, t826: F, t829: F, t830: F, t833: F, t844: F, t8659: F) -> F {
    let t50709 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t36340 + t326 * t49464 * t826 * t833 / F::cast_from(96.0_f64) + t13142 * t2503 / F::cast_from(24.0_f64) + t8659 * t829 * t830 * t2501 * t13290 / F::cast_from(12.0_f64) + F::cast_from(455.0_f64) / F::cast_from(162.0_f64) * t28413 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46892 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46914 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46928 + F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t39490 + F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t28487 - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t39510 - F::cast_from(35.0_f64) / F::cast_from(18.0_f64) * t39521 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t39523 - t844 * t338 * t353 * t376 * t48985 / F::cast_from(48.0_f64);
    t50709
}
