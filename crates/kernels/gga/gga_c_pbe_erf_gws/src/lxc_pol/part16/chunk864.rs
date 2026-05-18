//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 864/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk864<F: Float>(t11: F, t7385: F, t1758: F, t7346: F, t2704: F, t7336: F, t571: F, t7359: F, t7355: F, t7350: F, t1014: F, t1251: F) -> (F, F, F, F, F, F, F) {
    let t7386 = t11 * t7385;
    let t7388 = t1758 * t7346;
    let t7389 = t2704 * t7388;
    let t7391 = t1758 * t7336;
    let t7392 = t11 * t7391;
    let t7394 = t571 * t7359;
    let t7395 = t11 * t7394;
    let t7397 = t571 * t7355;
    let t7398 = t2704 * t7397;
    let t7400 = t571 * t7350;
    let t7401 = t11 * t7400;
    let t7407 = t1251 * t1014;
    (t7386, t7389, t7392, t7395, t7398, t7401, t7407)
}
