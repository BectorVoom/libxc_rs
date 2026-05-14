//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 211/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk211<F: Float>(t418: F, t591: F, t590: F, t587: F, t196: F) -> (F, F, F, F, F) {
    let t592 = t591 * t418;
    let t593 = t590 * t592;
    let t595 = 4.0 / 45.0 * t587 * t593;
    let t596 = t196 * t196;
    let t597 = 1.0 / t596;
    (t592, t593, t595, t596, t597)
}
