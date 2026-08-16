//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1204/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1204<F: Float>(t39400: F, t39410: F, t39385: F, t39387: F, t39390: F, t39393: F, t39397: F, t39403: F, t39406: F, t39413: F, t39416: F, t41367: F) -> F {
    let t41369 = F::cast_from(0.13869154784086829701e1_f64) * t39400;
    let t41372 = F::cast_from(0.95219938395347901946e-2_f64) * t39410;
    let t41375 = -F::cast_from(0.86682217400542685632e-1_f64) * t39385 - F::cast_from(0.17336443480108537126e0_f64) * t39387 + F::cast_from(0.17336443480108537126e0_f64) * t39390 + F::cast_from(0.5200933044032561138e0_f64) * t39393 + t41367 - F::cast_from(0.54878743191129263322e-1_f64) * t39397 - t41369 - F::cast_from(0.92461031893912198007e0_f64) * t39403 - F::cast_from(0.86682217400542685632e-1_f64) * t39406 + t41372 + F::cast_from(0.5200933044032561138e0_f64) * t39413 + F::cast_from(0.2600466522016280569e0_f64) * t39416;
    t41375
}
