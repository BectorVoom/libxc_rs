//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1027/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1027<F: Float>(t2100: F, t2121: F, t2122: F, t337: F, t6567: F, t814: F, t9465: F, t6401: F, t6684: F, t6688: F, t2189: F, t343: F, t816: F, t6402: F, t6633: F, t6183: F, t6341: F) -> (F, F, F, F, F, F) {
    let t20667 = t2121 * t337 * t2122 * t2100;
    let t20669 = t6567 * t20667 / 16.0;
    let t20670 = t9465 * t814;
    let t20675 = t6684 * t6401;
    let t20676 = t20675 * t6688;
    let t20682 = t816 * t2189 * t343;
    let t20687 = t6402 * t6633;
    let t20689 = t6183 * t6341;
    (t20669, t20670, t20676, t20682, t20687, t20689)
}
