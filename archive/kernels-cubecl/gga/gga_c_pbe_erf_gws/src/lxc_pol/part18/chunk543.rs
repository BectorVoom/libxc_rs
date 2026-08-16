//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 543/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk543<F: Float>(t11: F, t2765: F, t34: F, t572: F, t571: F, t2704: F, t1014: F, t401: F, t1856: F, t2561: F, t2555: F, t606: F) -> (F, F, F, F, F, F, F) {
    let t2766 = t11 * t2765;
    let t2768 = t572 * t34;
    let t2769 = t571 * t2768;
    let t2770 = t2704 * t2769;
    let t2773 = t401 * t1014;
    let t2775 = t1856 * t2561;
    let t2778 = t606 * t2555;
    (t2766, t2768, t2769, t2770, t2773, t2775, t2778)
}
