//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 641/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk641<F: Float>(t449: F, t5311: F, t438: F, t894: F, t3107: F, t5328: F, t123: F, t458: F, t429: F, t914: F, t1: F, t450: F, t464: F, t5255: F, t155: F, t146: F, t455: F, t5274: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t5412 = t5328 * t1 * t438;
    let t5413 = t450 * t5412;
    let t5416 = t464 * t5255;
    let t5417 = t155 * t5416;
    let t5421 = t146 * t455 * t5274;
    (t5388, t5389, t5392, t5393, t5394, t5398, t5399, t5403, t5404, t5407, t5408, t5412, t5413, t5416, t5417, t5421)
}
