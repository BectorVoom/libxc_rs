//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 752/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk752<F: Float>(t2132: F, t2271: F, t822: F, t362: F, t922: F, t2276: F, t932: F, t2315: F, t745: F, t810: F, t2306: F, t2382: F, t2074: F, t343: F, t2319: F, t2339: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6187 = t2271 * t2132;
    let t6188 = t822 * t6187;
    let t6201 = t362 * t922;
    let t6203 = t2276 * t6201 * t932;
    let t6204 = t6203 * t2315;
    let t6211 = t745 * t810;
    let t6216 = t2306 * t2132;
    let t6217 = t2382 * t6216;
    let t6220 = t343 * t2074;
    let t6225 = t2319 * t2339;
    (t6187, t6188, t6201, t6203, t6204, t6211, t6217, t6220, t6225)
}
