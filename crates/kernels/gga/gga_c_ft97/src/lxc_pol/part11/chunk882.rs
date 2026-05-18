//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 882/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk882<F: Float>(t1642: F, t37320: F, t92: F, t1557: F, t37362: F, t37269: F, t37357: F, t7800: F, t378: F, t3051: F, t380: F, t1652: F, t1771: F) -> (F, F, F, F, F, F, F, F) {
    let t38077 = t92 * t1642 * t37320;
    let t38079 = t1557 * t37362;
    let t38081 = t92 * t1642 * t38079;
    let t38084 = t92 * t1642 * t37269;
    let t38086 = t7800 * t37357;
    let t38088 = t92 * t378 * t38086;
    let t38090 = t3051 * t380;
    let t38092 = t1771 * t1652;
    (t38077, t38079, t38081, t38084, t38086, t38088, t38090, t38092)
}
