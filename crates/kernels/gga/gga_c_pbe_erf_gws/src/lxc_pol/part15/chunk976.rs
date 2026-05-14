//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 976/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk976<F: Float>(t13919: F, t2258: F, t13917: F, t1193: F, t814: F, t353: F, t859: F) -> (F, F, F, F, F) {
    let t13920 = t13919 * t2258;
    let t13921 = t13917 * t13920;
    let t13923 = t1193 * t814;
    let t13924 = t353 * t13923;
    let t13925 = t859 * t13924;
    (t13920, t13921, t13923, t13924, t13925)
}
