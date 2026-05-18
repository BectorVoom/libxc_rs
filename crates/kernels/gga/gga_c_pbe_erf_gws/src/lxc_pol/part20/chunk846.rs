//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 846/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk846<F: Float>(t2913: F, t5651: F, t2921: F, t475: F, t1076: F, t39: F, t2848: F, t532: F, t2522: F, t299: F, t169: F, t242: F) -> (F, F, F, F, F) {
    let t8332 = t5651 * t2913;
    let t8341 = t475 * t2921;
    let t8347 = t39 * t1076;
    let t8351 = F::new(0.2133002709687175212e0) * t532 * t2848;
    let t8352 = t299 * t2522;
    let t8355 = F::new(0.1061188859155979109e0) * t169 * t8352 * t242;
    (t8332, t8341, t8347, t8351, t8355)
}
