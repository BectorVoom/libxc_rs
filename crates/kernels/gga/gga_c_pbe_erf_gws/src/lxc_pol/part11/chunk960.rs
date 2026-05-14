//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 960/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk960<F: Float>(t47400: F, t572: F, t47391: F, t4951: F, t11: F, t1758: F, t3346: F) -> (F, F, F, F) {
    let t47401 = t572 * t47400;
    let t47405 = t4951 * t47391;
    let t47407 = t11 * t1758 * t47405;
    let t47409 = t3346 * t3346;
    (t47401, t47405, t47407, t47409)
}
