//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 502/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk502<F: Float>(t2376: F, t938: F, t830: F, t829: F, t2306: F, t825: F, t2271: F, t376: F, t891: F) -> (F, F, F, F, F) {
    let t2377 = t2376 * t938;
    let t2378 = t830 * t2377;
    let t2379 = t829 * t2378;
    let t2383 = t2306 * t825;
    let t2391 = t2271 * t825;
    let t2395 = t891 * t376;
    (t2377, t2379, t2383, t2391, t2395)
}
