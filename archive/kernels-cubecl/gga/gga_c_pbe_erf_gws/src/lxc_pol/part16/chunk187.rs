//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 187/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk187<F: Float>(t138: F, t510: F, t514: F, t520: F, t101: F, t131: F, t137: F) -> (F, F, F, F) {
    let t522 = t138 * t510 - t514 * t520;
    let t523 = t101 * t522;
    let t524 = F::cast_from(1.0_f64) / t131;
    let t525 = t524 * t137;
    (t522, t523, t524, t525)
}
