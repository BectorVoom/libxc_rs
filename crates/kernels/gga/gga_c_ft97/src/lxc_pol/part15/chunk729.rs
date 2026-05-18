//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 729/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk729<F: Float>(t167: F, t20714: F, t2185: F, t3578: F, t4733: F, t574: F, t1053: F, t4714: F, t605: F, t1017: F, t4805: F, t4724: F) -> (F, F, F, F, F, F, F) {
    let t20716 = t2185 * t167 * t20714;
    let t20720 = t574 * t3578 * t4733;
    let t20723 = t4714 * t1053;
    let t20725 = t574 * t605 * t20723;
    let t20727 = t1017 * t4805;
    let t20729 = t574 * t605 * t20727;
    let t20731 = t4724 * t1017;
    (t20716, t20720, t20723, t20725, t20727, t20729, t20731)
}
