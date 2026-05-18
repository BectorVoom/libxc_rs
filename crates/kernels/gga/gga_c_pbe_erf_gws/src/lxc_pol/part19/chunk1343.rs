//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1343/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1343<F: Float>(t1109: F, t1113: F, t3972: F, t3975: F, t4408: F, t2409: F, t35910: F, t3965: F, t13808: F, t15146: F, t15191: F, t50994: F) -> (F, F, F, F) {
    let t57635 = t3972 * t3975 * t1113 * t4408 * t1109;
    let t57639 = t3965 * t2409 * t35910;
    let t57641 = t13808 * t15146;
    let t57643 = t50994 * t15191;
    (t57635, t57639, t57641, t57643)
}
