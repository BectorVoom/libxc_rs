//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 725/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk725<F: Float>(t185: F, t5357: F, t5081: F, t172: F, t1773: F, t184: F, t1903: F, t720: F, t254: F, t542: F, t252: F, t1907: F, t723: F, t1697: F, t212: F, t22: F) -> (F, F, F, F, F, F, F) {
    let t5359 = 16.0 / 405.0 * t185 * t5357;
    let t5360 = 0.58774074074074074074e-2 * t5081;
    let t5378 = t172 * t1773;
    let t5379 = t5378 * t184;
    let t5384 = 2.0 / 9.0 * t720 * t1903;
    let t5385 = t254 * t542;
    let t5387 = 8.0 / 81.0 * t252 * t5385;
    let t5388 = t1907 * t723;
    let t5399 = 1.0 / t212 / t1697;
    let t5400 = t22 * t5399;
    (t5359, t5360, t5379, t5384, t5387, t5388, t5400)
}
