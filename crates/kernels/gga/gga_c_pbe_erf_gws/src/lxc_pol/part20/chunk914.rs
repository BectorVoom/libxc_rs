//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 914/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk914<F: Float>(t2030: F, t985: F, t2032: F, t299: F, t3379: F, t169: F, t242: F, t10201: F, t171: F, t3689: F, t700: F, t3373: F, t532: F) -> (F, F, F, F, F) {
    let t10222 = t2030 * t985;
    let t10223 = t10222 * t2032;
    let t10229 = t299 * t3379;
    let t10231 = t169 * t10229 * t242;
    let t10233 = t171 * t10201;
    let t10239 = t169 * t3689 * t700;
    let t10245 = t532 * t3373;
    (t10223, t10231, t10233, t10239, t10245)
}
