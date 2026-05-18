//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1078/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1078<F: Float>(t12098: F, t898: F, t338: F, t353: F, t3067: F, t3721: F, t829: F, t830: F, t1118: F, t3200: F, t1144: F, t3097: F) -> (F, F, F, F, F, F) {
    let t12099 = t898 * t12098;
    let t12101 = t338 * t353 * t12099;
    let t12109 = t3067 * t3721;
    let t12111 = t829 * t830 * t12109;
    let t12121 = t338 * t3200 * t1118;
    let t12125 = t338 * t1144 * t3097;
    (t12099, t12101, t12109, t12111, t12121, t12125)
}
