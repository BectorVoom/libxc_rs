//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 707/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk707<F: Float>(t10207: F, t281: F, t2030: F, t985: F, t299: F, t3379: F, t169: F, t242: F, t3689: F, t700: F, t3373: F, t532: F) -> (F, F, F, F, F, F) {
    let t10208 = t281 * t10207;
    let t10222 = t2030 * t985;
    let t10229 = t299 * t3379;
    let t10231 = t169 * t10229 * t242;
    let t10239 = t169 * t3689 * t700;
    let t10245 = t532 * t3373;
    (t10208, t10222, t10229, t10231, t10239, t10245)
}
