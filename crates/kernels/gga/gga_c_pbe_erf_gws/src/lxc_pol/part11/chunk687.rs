//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 687/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk687<F: Float>(t11609: F, t2306: F, t3717: F, t6: F, t11478: F, t343: F, t337: F, t2121: F, t346: F, t9847: F, t1114: F, t254: F) -> (F, F, F, F, F, F) {
    let t11610 = t2306 * t11609;
    let t11618 = t6 * t3717;
    let t11628 = t11478 * t343;
    let t11629 = t337 * t11628;
    let t11630 = t2121 * t11629;
    let t11667 = t9847 * t346;
    let t11668 = t1114 * t11667;
    let t11700 = t254 * t11618;
    (t11610, t11629, t11630, t11667, t11668, t11700)
}
