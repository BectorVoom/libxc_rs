//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 804/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk804<F: Float>(t2325: F, t31501: F, t882: F, t883: F, t2321: F, t34604: F, t9074: F, t10687: F, t2554: F, t7064: F, t13200: F, t29439: F) -> (F, F, F, F) {
    let t42889 = t882 * t2325 * t883 * t31501;
    let t42898 = t9074 * t34604 * t2321;
    let t42931 = t7064 * t10687 * t2554;
    let t42933 = t29439 * t13200;
    (t42889, t42898, t42931, t42933)
}
