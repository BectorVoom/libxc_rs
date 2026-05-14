//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1088/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1088<F: Float>(t82518: F, t82561: F, t1196: F, t283: F, t4093: F, t1471: F, t22081: F, t5295: F, t811: F, t820: F, t2035: F, t39: F, t5284: F, t1208: F, t4125: F, t287: F, t5231: F) -> (F, F, F, F, F, F, F, F, F) {
    let t82562 = t82518 + t82561;
    let t82816 = t1196 * t283;
    let t82817 = t82816 * t4093;
    let t82822 = t22081 * t1471;
    let t82940 = t5295 * t811;
    let t82944 = t5295 * t820;
    let t82957 = t811 * t39 * t2035;
    let t82960 = t5284 * t820;
    let t83145 = t1208 * t4125;
    let t83310 = t5231 * t287;
    (t82562, t82817, t82822, t82940, t82944, t82957, t82960, t83145, t83310)
}
