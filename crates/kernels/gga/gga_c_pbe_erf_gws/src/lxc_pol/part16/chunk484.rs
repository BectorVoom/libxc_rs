//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 484/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk484<F: Float>(t5: F, t814: F, t337: F, t2121: F, t2120: F, t339: F, t745: F, t850: F, t851: F) -> (F, F, F, F, F, F) {
    let t2122 = t5 * t814;
    let t2123 = t337 * t2122;
    let t2124 = t2121 * t2123;
    let t2126 = t2120 * t2124 / 96.0;
    let t2127 = t745 * t339;
    let t2129 = t850 * t851 * t2127;
    (t2122, t2123, t2124, t2126, t2127, t2129)
}
