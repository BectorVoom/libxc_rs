//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1351/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1351<F: Float>(t11439: F, t54047: F, t11746: F, t51351: F, t11431: F, t51306: F, t11854: F, t14031: F, t11860: F, t4028: F, t11919: F, t4049: F) -> (F, F, F, F, F, F) {
    let t57225 = t54047 * t11439;
    let t57227 = t51351 * t11746;
    let t57229 = t51306 * t11431;
    let t57231 = t14031 * t11854;
    let t57233 = t4028 * t11860;
    let t57235 = t4049 * t11919;
    (t57225, t57227, t57229, t57231, t57233, t57235)
}
