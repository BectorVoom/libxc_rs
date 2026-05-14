//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 763/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk763<F: Float>(t1255: F, t5299: F, t840: F, t22161: F, t319: F, t1212: F, t5424: F, t1091: F, t19576: F, t2874: F, t19571: F, t2881: F, t1248: F, t19333: F, t296: F, t10749: F, t15329: F, t15420: F, t1901: F, t193: F, t22398: F, t22402: F, t22407: F, t22412: F, t22416: F, t22441: F, t446: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22446 = t840 * t1255 * t5299;
    let t22449 = t840 * t319 * t22161;
    let t22454 = t840 * t5424 * t1212;
    let t22456 = t19576 * t1091;
    let t22457 = t2874 * t22456;
    let t22460 = t19571 * t1091;
    let t22461 = t2881 * t22460;
    let t22464 = t19333 * t1248;
    let t22465 = t296 * t22464;
    let t22467 = 2.0 / 3.0 * t1901 * t22398 - t446 * t22402 / 3.0 + 2.0 * t446 * t22407 - 2.0 * t446 * t22412 - 2.0 / 3.0 * t446 * t22416 + 4.0 / 9.0 * t15329 + t89 * t193 * t22441 / 3.0 - t446 * t22446 - t446 * t22449 / 3.0 - 4.0 / 9.0 * t15420 - t446 * t22454 + t1901 * t22457 / 3.0 + t1901 * t22461 / 3.0 - t10749 - t446 * t22465;
    (t22446, t22449, t22454, t22456, t22457, t22460, t22461, t22464, t22465, t22467)
}
