//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 803/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk803<F: Float>(t2365: F, t828: F, t2137: F, t2134: F, t2132: F, t2271: F, t822: F, t362: F, t922: F, t2276: F, t932: F, t2315: F) -> (F, F, F, F, F, F, F, F) {
    let t6183 = t2365 * t828;
    let t6184 = t6183 * t2137;
    let t6185 = t2134 * t6184;
    let t6187 = t2271 * t2132;
    let t6188 = t822 * t6187;
    let t6201 = t362 * t922;
    let t6203 = t2276 * t6201 * t932;
    let t6204 = t6203 * t2315;
    (t6183, t6184, t6185, t6187, t6188, t6201, t6203, t6204)
}
