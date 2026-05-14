//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 471/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk471<F: Float>(t2120: F, t2124: F, t339: F, t745: F, t850: F, t851: F, t860: F, t360: F) -> (F, F, F, F, F) {
    let t2126 = t2120 * t2124 / 96.0;
    let t2127 = t745 * t339;
    let t2129 = t850 * t851 * t2127;
    let t2131 = t2129 * t860 / 96.0;
    let t2132 = t339 * t360;
    (t2126, t2127, t2129, t2131, t2132)
}
