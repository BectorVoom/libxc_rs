//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 863/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk863<F: Float>(t1527: F, t37315: F, t419: F, t37264: F, t3088: F, t37320: F, t1725: F, t8093: F, t7705: F, t7789: F, t8106: F, t11262: F, t7807: F) -> (F, F, F, F, F, F, F) {
    let t37696 = t419 * t1527 * t37315;
    let t37699 = t419 * t1527 * t37264;
    let t37702 = t419 * t3088 * t37320;
    let t37704 = t1725 * t8093;
    let t37707 = t419 * t7705 * t7789;
    let t37709 = t1725 * t8106;
    let t37712 = t419 * t11262 * t7807;
    (t37696, t37699, t37702, t37704, t37707, t37709, t37712)
}
