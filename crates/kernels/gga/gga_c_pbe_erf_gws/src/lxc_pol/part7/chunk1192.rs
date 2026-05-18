//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1192/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1192<F: Float>(t6484: F, t6526: F, t2127: F, t6619: F, t850: F, t860: F, t346: F, t6110: F, t822: F, t2150: F, t2083: F, t6104: F) -> (F, F, F, F) {
    let t21182 = t6484 * t6526;
    let t21183 = F::new(7.0) / F::new(4.0) * t21182;
    let t21187 = t850 * t6619 * t2127 * t860 / F::new(32.0);
    let t21188 = t6110 * t346;
    let t21189 = t822 * t21188;
    let t21191 = t21189 * t2150 / F::new(12.0);
    let t21196 = t6104 * t2083;
    (t21183, t21187, t21191, t21196)
}
