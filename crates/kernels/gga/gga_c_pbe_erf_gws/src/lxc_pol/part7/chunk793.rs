//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 793/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk793<F: Float>(t346: F, t6158: F, t822: F, t5: F, t6161: F, t337: F, t2121: F, t2100: F, t274: F, t2255: F, t2278: F, t2251: F, t2299: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6566 = t6158 * t346;
    let t6567 = t822 * t6566;
    let t6568 = t5 * t6161;
    let t6569 = t337 * t6568;
    let t6570 = t2121 * t6569;
    let t6572 = t6567 * t6570 / F::new(48.0);
    let t6573 = t274 * t2100;
    let t6575 = t2255 * t2278 * t6573;
    let t6578 = t2251 * t2299;
    (t6566, t6567, t6568, t6569, t6570, t6572, t6573, t6575, t6578)
}
