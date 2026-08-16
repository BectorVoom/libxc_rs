//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 659/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk659<F: Float>(t449: F, t5311: F, t438: F, t894: F, t3107: F, t5328: F, t123: F, t458: F, t429: F, t914: F, t1: F, t450: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5387 = t449 * t5311;
    let t5388 = t5387 * t438;
    let t5389 = t894 * t5388;
    let t5392 = t5328 * t3107;
    let t5393 = t5392 * t123;
    let t5394 = t458 * t5393;
    let t5398 = t5328 * t123 * t438;
    let t5399 = t458 * t5398;
    let t5403 = t429 * t5311 * t438;
    let t5404 = t914 * t5403;
    let t5407 = t5392 * t1;
    let t5408 = t450 * t5407;
    (t5388, t5389, t5392, t5393, t5394, t5398, t5399, t5403, t5404, t5407, t5408)
}
