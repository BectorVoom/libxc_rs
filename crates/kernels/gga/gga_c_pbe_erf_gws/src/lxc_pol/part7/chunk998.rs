//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 998/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk998<F: Float>(t20112: F, t822: F, t19745: F, t2306: F, t3074: F, t19751: F, t2118: F, t2382: F, t4384: F, t353: F, t745: F, t859: F, t939: F, t19592: F, t20081: F, t20086: F, t20092: F, t20106: F, t20108: F, t20110: F, t2074: F, t2373: F, t2408: F, t2409: F, t2417: F, t3067: F, t335: F, t338: F, t4390: F, t6724: F, t6797: F, t6816: F, t6817: F, t833: F, t892: F) -> (F,) {
    let t20113 = t822 * t20112;
    let t20117 = t3074 * t2306 * t19745;
    let t20121 = t2382 * t2118 * t19751;
    let t20124 = t2382 * t4384;
    let t20127 = t859 * t353 * t939 * t745;
    let t20130 = -455.0 / 324.0 * t20081 + t19592 * t4390 / 6.0 + t2382 * t20086 * t833 / 32.0 + 35.0 / 18.0 * t20092 - t2408 * t2409 * t3067 * t2074 * t2417 / 4.0 - t335 * t338 * t892 * t6724 / 24.0 - t6816 * t338 * t892 * t6817 - 35.0 / 18.0 * t20106 - 7.0 / 6.0 * t20108 - 7.0 / 12.0 * t20110 + t20113 * t6797 / 4.0 - t20117 * t2373 / 4.0 - t20121 * t2373 / 4.0 - t20124 * t20127 / 8.0;
    (t20130,)
}
