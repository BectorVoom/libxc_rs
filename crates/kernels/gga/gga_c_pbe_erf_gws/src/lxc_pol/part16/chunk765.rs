//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 765/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk765<F: Float>(t1778: F, t633: F, t198: F, t2735: F, t185: F, t5081: F, t172: F, t1773: F, t184: F, t1903: F, t720: F, t254: F, t542: F) -> (F, F, F, F, F, F, F) {
    let t5355 = t633 * t1778;
    let t5357 = t2735 * t198;
    let t5359 = F::new(16.0) / F::new(405.0) * t185 * t5357;
    let t5360 = F::new(0.58774074074074074074e-2) * t5081;
    let t5378 = t172 * t1773;
    let t5379 = t5378 * t184;
    let t5384 = F::new(2.0) / F::new(9.0) * t720 * t1903;
    let t5385 = t254 * t542;
    (t5355, t5357, t5359, t5360, t5379, t5384, t5385)
}
