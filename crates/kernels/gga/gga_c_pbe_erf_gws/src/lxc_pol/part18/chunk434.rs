//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 434/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk434<F: Float>(t191: F, t299: F, t190: F, t212: F, t401: F, t658: F, t204: F, t205: F) -> (F, F, F, F) {
    let t1706 = t299 * t191;
    let t1709 = F::new(0.11111111111111111111e-1) * t190 * t1706 * t212;
    let t1710 = t401 * t658;
    let t1713 = F::new(1.0) / t205 / t204;
    let t1714 = t191 * t1713;
    (t1706, t1709, t1710, t1714)
}
