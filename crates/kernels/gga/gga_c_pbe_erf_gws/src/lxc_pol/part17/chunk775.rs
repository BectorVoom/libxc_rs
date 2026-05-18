//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 775/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk775<F: Float>(t2031: F, t5628: F, t168: F, t5589: F, t286: F, t2030: F, t522: F, t475: F, t137: F, t142: F, t481: F, t510: F) -> (F, F, F, F, F) {
    let t5629 = t2031 * t5628;
    let t5631 = t168 * t5589;
    let t5633 = F::new(0.19513566535229733338e0) * t5631 * t286;
    let t5649 = t522 * t2030;
    let t5650 = t475 * t5649;
    let t5651 = t137 * t142;
    let t5652 = t510 * t481;
    (t5629, t5633, t5650, t5651, t5652)
}
