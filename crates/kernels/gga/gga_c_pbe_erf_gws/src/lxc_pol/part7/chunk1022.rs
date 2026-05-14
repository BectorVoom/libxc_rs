//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1022/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1022<F: Float>(t20583: F, t19562: F, t346: F, t2124: F, t6800: F, t2147: F, t337: F, t6568: F, t810: F, t6567: F, t2319: F, t6474: F, t2189: F, t343: F, t814: F, t3065: F, t858: F) -> (F, F, F, F, F, F) {
    let t20584 = 7.0 / 12.0 * t20583;
    let t20585 = t19562 * t346;
    let t20588 = t6800 * t20585 * t2124 / 16.0;
    let t20591 = t2147 * t337 * t6568 * t810;
    let t20593 = t6567 * t20591 / 6.0;
    let t20594 = t2319 * t6474;
    let t20597 = t814 * t2189 * t343;
    let t20599 = t3065 * t858 * t20597;
    (t20584, t20588, t20593, t20594, t20597, t20599)
}
