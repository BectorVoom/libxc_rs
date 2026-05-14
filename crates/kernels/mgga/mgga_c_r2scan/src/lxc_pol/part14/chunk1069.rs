//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1069/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1069<F: Float>(t39395: F, t39400: F, t39410: F, t39385: F, t39387: F, t39390: F, t39393: F, t39397: F, t39403: F, t39406: F, t39413: F, t39416: F, t39437: F, t39440: F, t39443: F, t39445: F) -> (F, F, F, F, F) {
    let t41367 = 0.25610080155860322884e0 * t39395;
    let t41369 = 0.13869154784086829701e1 * t39400;
    let t41372 = 0.95219938395347901946e-2 * t39410;
    let t41375 = -0.86682217400542685632e-1 * t39385 - 0.17336443480108537126e0 * t39387 + 0.17336443480108537126e0 * t39390 + 0.5200933044032561138e0 * t39393 + t41367 - 0.54878743191129263322e-1 * t39397 - t41369 - 0.92461031893912198007e0 * t39403 - 0.86682217400542685632e-1 * t39406 + t41372 + 0.5200933044032561138e0 * t39413 + 0.2600466522016280569e0 * t39416;
    let t41384 = 0.95219938395347901946e-2 * t39437;
    let t41385 = 0.19043987679069580389e-1 * t39440;
    let t41386 = 0.28565981518604370584e-1 * t39443;
    let t41387 = 0.95219938395347901946e-2 * t39445;
    (t41375, t41384, t41385, t41386, t41387)
}
