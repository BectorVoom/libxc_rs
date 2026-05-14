//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1027/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1027<F: Float>(t14611: F, t1161: F, t4110: F, t2409: F, t3067: F, t14283: F, t14609: F, t14615: F, t14618: F, t14902: F, t14906: F, t14911: F, t14914: F, t14918: F, t14924: F, t14928: F, t2408: F, t3066: F, t335: F, t827: F) -> (F, F, F) {
    let t14931 = 7.0 / 2304.0 * t14611;
    let t14935 = t4110 * t1161;
    let t14937 = t2409 * t3067 * t14935;
    let t14940 = t3066 * t14902 / 48.0 + t2408 * t14906 / 48.0 - t827 * t14911 / 96.0 + 7.0 / 288.0 * t14914 - t827 * t14918 / 96.0 - t14609 / 1536.0 + t2408 * t14924 / 48.0 - t335 * t14928 / 96.0 + t14931 - t14615 / 384.0 + 7.0 / 288.0 * t14283 - t14618 / 48.0 + t3066 * t14937 / 48.0;
    (t14935, t14937, t14940)
}
