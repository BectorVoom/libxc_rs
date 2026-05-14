//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 741/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk741<F: Float>(t2173: F, t6484: F, t2113: F, t2127: F, t850: F, t860: F, t1452: F, t339: F, t851: F, t6440: F, t904: F, t916: F, t2264: F, t899: F, t922: F, t2268: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6485 = t6484 * t2173;
    let t6486 = 7.0 / 24.0 * t6485;
    let t6488 = t850 * t2113 * t2127;
    let t6490 = t6488 * t860 / 48.0;
    let t6491 = t1452 * t339;
    let t6493 = t850 * t851 * t6491;
    let t6495 = t6493 * t860 / 96.0;
    let t6497 = t916 * t904 * t6440;
    let t6501 = t899 * t2264 * t922;
    let t6502 = t6501 * t2268;
    (t6486, t6488, t6490, t6491, t6493, t6495, t6497, t6501, t6502)
}
