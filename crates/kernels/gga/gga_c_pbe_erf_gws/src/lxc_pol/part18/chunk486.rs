//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 486/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk486<F: Float>(t2118: F, t825: F, t814: F, t830: F, t831: F, t829: F, t328: F, t837: F) -> (F, F, F) {
    let t2358 = t2118 * t825;
    let t2361 = t830 * t831 * t814;
    let t2362 = t829 * t2361;
    let t2365 = t328 * t837;
    (t2358, t2362, t2365)
}
