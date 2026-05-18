//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 382/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk382<F: Float>(t510: F, t513: F, t137: F, t512: F, t131: F, t520: F) -> (F, F, F, F) {
    let t1572 = t510 * t513;
    let t1576 = F::new(1.0) / t512 / t137;
    let t1577 = t131 * t1576;
    let t1578 = t520 * t520;
    (t1572, t1576, t1577, t1578)
}
