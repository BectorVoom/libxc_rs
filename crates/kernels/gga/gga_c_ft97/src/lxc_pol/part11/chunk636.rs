//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 636/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk636<F: Float>(t2221: F, t9424: F, t2133: F, t604: F, t609: F, t144: F, t24: F, t7368: F, t167: F, t9017: F, t603: F, t157: F, t2180: F, t1986: F, t2185: F, t616: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9425 = t2221 * t9424;
    let t9428 = t2133 * t604;
    let t9429 = t9428 * t609;
    let t9430 = t144 * t9429;
    let t9432 = t24 * t7368;
    let t9434 = t9432 * t167 * t9017;
    let t9437 = t603 * t603;
    let t9438 = 1.0 / t9437;
    let t9439 = t157 * t9438;
    let t9440 = t2180 * t609;
    let t9441 = t9439 * t9440;
    let t9442 = t144 * t9441;
    let t9446 = t2185 * t616 * t1986;
    (t9425, t9428, t9429, t9430, t9432, t9434, t9437, t9438, t9439, t9440, t9441, t9442, t9446)
}
