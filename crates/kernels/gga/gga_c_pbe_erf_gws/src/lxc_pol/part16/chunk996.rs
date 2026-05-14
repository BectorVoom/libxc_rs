//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 996/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk996<F: Float>(t4110: F, t810: F, t2376: F, t2409: F, t1205: F, t2417: F, t9296: F, t938: F, t3067: F, t338: F, t4111: F, t892: F, t4094: F, t840: F, t1206: F, t2220: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14258 = t4110 * t810;
    let t14260 = t2409 * t2376 * t14258;
    let t14264 = t1205 * t2417;
    let t14266 = t2409 * t9296 * t14264;
    let t14272 = t4110 * t938;
    let t14274 = t2409 * t3067 * t14272;
    let t14280 = t338 * t892 * t4111;
    let t14283 = t840 * t4094;
    let t14286 = t338 * t2220 * t1206;
    (t14258, t14260, t14264, t14266, t14272, t14274, t14280, t14283, t14286)
}
