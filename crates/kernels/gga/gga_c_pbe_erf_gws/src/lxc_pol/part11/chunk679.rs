//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 679/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk679<F: Float>(t1651: F, t3526: F, t587: F, t3390: F, t626: F, t3399: F, t583: F, t1802: F, t3443: F, t5109: F, t3380: F, t700: F, t145: F, t3379: F) -> (F, F, F, F, F, F, F, F) {
    let t11037 = t1651 * t3526;
    let t11038 = t587 * t11037;
    let t11054 = t3390 * t626;
    let t11065 = t3399 * t583;
    let t11110 = t1802 * t3443;
    let t11136 = t5109 * t3390;
    let t11157 = t3380 * t700;
    let t11159 = t145 * t3379;
    (t11037, t11038, t11054, t11065, t11110, t11136, t11157, t11159)
}
