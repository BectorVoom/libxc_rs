//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1093/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1093<F: Float>(t376: F, t3906: F, t829: F, t830: F, t3703: F, t938: F, t2376: F, t2409: F, t11737: F, t831: F, t3733: F, t8662: F) -> (F, F, F, F, F, F) {
    let t12232 = t3906 * t376;
    let t12234 = t829 * t830 * t12232;
    let t12237 = t3703 * t938;
    let t12239 = t2409 * t2376 * t12237;
    let t12243 = t2409 * t831 * t11737;
    let t12246 = t8662 * t3733;
    (t12232, t12234, t12237, t12239, t12243, t12246)
}
