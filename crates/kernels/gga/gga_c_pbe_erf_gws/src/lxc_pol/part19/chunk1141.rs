//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1141/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1141<F: Float>(t3972: F, t54499: F, t54590: F, t8884: F, t13984: F, t56104: F, t11438: F, t13917: F, t2249: F, t53446: F, t3765: F, t51465: F, t11414: F, t2134: F, t12021: F, t14031: F) -> (F, F, F, F, F, F) {
    let t56847 = t3972 * t54499 * t8884 * t54590;
    let t56849 = t56104 * t13984;
    let t56853 = t13917 * t2249 * t53446 * t11438;
    let t56855 = t51465 * t3765;
    let t56857 = t2134 * t11414;
    let t56859 = t14031 * t12021;
    (t56847, t56849, t56853, t56855, t56857, t56859)
}
