//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 783/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk783<F: Float>(t551: F, t8308: F, t1480: F, t1076: F, t169: F, t301: F, t366: F, t1500: F, t2858: F, t2913: F, t5651: F, t2921: F, t475: F, t39: F, t2848: F, t532: F) -> (F, F, F, F, F, F, F) {
    let t8309 = t8308 * t551;
    let t8310 = t8309 * t1480;
    let t8314 = t169 * t366 * t1076 * t301;
    let t8318 = t1500 * t2858;
    let t8332 = t5651 * t2913;
    let t8341 = t475 * t2921;
    let t8347 = t39 * t1076;
    let t8351 = 0.2133002709687175212e0 * t532 * t2848;
    (t8310, t8314, t8318, t8332, t8341, t8347, t8351)
}
