//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1204/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1204(t39400: f64, t39410: f64, t39385: f64, t39387: f64, t39390: f64, t39393: f64, t39397: f64, t39403: f64, t39406: f64, t39413: f64, t39416: f64, t41367: f64) -> f64 {
    let t41369 = 0.13869154784086829701e1_f64 * t39400;
    let t41372 = 0.95219938395347901946e-2_f64 * t39410;
    let t41375 = -0.86682217400542685632e-1_f64 * t39385 - 0.17336443480108537126e0_f64 * t39387 + 0.17336443480108537126e0_f64 * t39390 + 0.5200933044032561138e0_f64 * t39393 + t41367 - 0.54878743191129263322e-1_f64 * t39397 - t41369 - 0.92461031893912198007e0_f64 * t39403 - 0.86682217400542685632e-1_f64 * t39406 + t41372 + 0.5200933044032561138e0_f64 * t39413 + 0.2600466522016280569e0_f64 * t39416;
    t41375
}
