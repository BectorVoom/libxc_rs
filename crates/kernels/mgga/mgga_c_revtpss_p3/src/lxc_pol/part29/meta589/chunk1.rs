//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1952/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1952<F: Float>(t28154: F, t95296: F, t28147: F, t95319: F, t28150: F, t7348: F, t25162: F, t101200: F, t101204: F, t101230: F, t101234: F, t101252: F, t101399: F, t26175: F, t26182: F, t28628: F, t92565: F, t95276: F, t95306: F, t95316: F, t95340: F) -> F {
    let t101955 = F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t28154 * t95296;
    let t101969 = F::cast_from(160.0_f64) / F::cast_from(3.0_f64) * t95319 * t28147;
    let t101970 = t7348 * t28150;
    let t101972 = F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t25162 * t101970;
    let t101975 = F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t95306 - F::cast_from(20.0_f64) * t101252 * t95340 - t101955 + F::cast_from(20.0_f64) * t95276 * t28147 + F::cast_from(20.0_f64) * t26175 * t101399 + F::cast_from(20.0_f64) * t26175 * t101200 + F::cast_from(10.0_f64) * t26175 * t101204 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t101230 * t26182 - F::cast_from(70.0_f64) * t95316 * t101234 - t101969 - t101972 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t92565 * t28628;
    t101975
}
