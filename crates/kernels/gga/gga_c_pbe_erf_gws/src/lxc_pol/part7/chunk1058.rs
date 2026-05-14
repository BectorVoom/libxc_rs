//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1058/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1058<F: Float>(t6484: F, t6526: F, t2127: F, t6619: F, t850: F, t860: F, t346: F, t6110: F, t822: F, t2150: F, t2083: F, t6104: F, t6161: F, t745: F, t2100: F, t1452: F, t274: F) -> (F, F, F, F, F, F, F) {
    let t21182 = t6484 * t6526;
    let t21183 = 7.0 / 4.0 * t21182;
    let t21187 = t850 * t6619 * t2127 * t860 / 32.0;
    let t21188 = t6110 * t346;
    let t21189 = t822 * t21188;
    let t21191 = t21189 * t2150 / 12.0;
    let t21196 = t6104 * t2083;
    let t21201 = t745 * t6161;
    let t21206 = t745 * t2100;
    let t21211 = t1452 * t274;
    (t21183, t21187, t21191, t21196, t21201, t21206, t21211)
}
