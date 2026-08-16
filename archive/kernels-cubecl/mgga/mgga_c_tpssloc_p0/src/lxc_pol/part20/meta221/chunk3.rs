//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1294/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1294<F: Float>(t9218: F, t2230: F, t594: F, t2229: F, t3: F, t19: F, t9211: F, t9213: F, t9215: F, t9217: F, t2233: F, t604: F) -> (F, F, F, F, F) {
    let t9219 = F::cast_from(0.12804e4_f64) * t9218;
    let t9220 = t594 * t2230;
    let t9221 = F::cast_from(0.170856e4_f64) * t9220;
    let t9222 = t2229 * t3;
    let t9223 = F::cast_from(1.0_f64) / t9222;
    let t9225 = F::cast_from(0.75936e3_f64) * t19 * t9223;
    let t9226 = -t9211 + t9213 - t9215 + t9217 - t9219 + t9221 - t9225;
    let t9228 = t2233 * t604;
    (t9220, t9223, t9225, t9226, t9228)
}
