//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 619/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk619<F: Float>(t157: F, t9132: F, t160: F, t1986: F, t379: F, t2178: F, t2180: F, t2210: F, t2101: F, t605: F) -> (F, F, F, F, F, F) {
    let t9133 = t9132 * t157;
    let t9135 = t160 * t1986 * t379;
    let t9136 = t9133 * t9135;
    let t9140 = t2178 * t2180 * t379;
    let t9141 = t2210 * t9140;
    let t9144 = t2101 * t605;
    (t9133, t9135, t9136, t9140, t9141, t9144)
}
