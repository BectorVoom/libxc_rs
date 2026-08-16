//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1031/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1031<F: Float>(t1161: F, t2417: F, t2409: F, t9296: F, t3076: F, t8945: F, t1118: F, t2220: F, t338: F, t3200: F, t845: F, t3097: F, t892: F) -> (F, F, F, F, F, F) {
    let t9297 = t1161 * t2417;
    let t9299 = t2409 * t9296 * t9297;
    let t9302 = t8945 * t3076;
    let t9307 = t338 * t2220 * t1118;
    let t9313 = t338 * t3200 * t845;
    let t9317 = t338 * t892 * t3097;
    (t9297, t9299, t9302, t9307, t9313, t9317)
}
