//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 165/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk165<F: Float>(t40: F, t428: F, t67: F, t62: F, t393: F, t395: F, t399: F, t401: F, t70: F) -> (F, F, F, F, F, F) {
    let t429 = t40 * t428;
    let t433 = t67 * t67;
    let t434 = F::new(1.0) / t433;
    let t435 = t62 * t434;
    let t440 = -F::new(0.1176575e1) * t393 - F::new(0.516475e0) * t395 - F::new(0.2103875e0) * t399 - F::new(0.104195e0) * t401;
    let t441 = F::new(1.0) / t70;
    (t429, t433, t434, t435, t440, t441)
}
