//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 721/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk721<F: Float>(t120: F, t4466: F, t19977: F, t378: F, t20049: F, t528: F, t19993: F, t72: F, t8965: F, t920: F, t16854: F, t126: F) -> (F, F, F, F, F, F, F, F) {
    let t20592 = t120 * t4466;
    let t20596 = t378 * t19977 * t120;
    let t20599 = t528 * t20049;
    let t20603 = t72 * t19993 * t120;
    let t20606 = t8965 * t920;
    let t20607 = t16854 * t20606;
    let t20612 = t72 * t19977 * t528 * t120;
    let t20615 = t19977 * t126;
    (t20592, t20596, t20599, t20603, t20606, t20607, t20612, t20615)
}
