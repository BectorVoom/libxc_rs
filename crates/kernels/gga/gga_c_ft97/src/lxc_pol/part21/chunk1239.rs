//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1239/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1239<F: Float>(t22591: F, t26738: F, t3404: F, t1013: F, t104923: F, t26743: F, t105157: F, t93178: F, t115516: F, t5838: F, t4702: F, t58: F, t538: F, t104637: F, t25713: F, t93048: F) -> (F, F, F, F, F, F, F, F) {
    let t118930 = t22591 * t26738 * t3404;
    let t118934 = t22591 * t104923 * t1013;
    let t118938 = t22591 * t26743 * t3404;
    let t118942 = t93178 * t105157 * t1013;
    let t118954 = t5838 * t115516;
    let t118968 = t58 * t4702;
    let t118970 = t22591 * t118968 * t538;
    let t118976 = t93048 * t104637 * t25713;
    (t118930, t118934, t118938, t118942, t118954, t118968, t118970, t118976)
}
