//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 462/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk462<F: Float>(t159: F, t285: F, t3013: F, t1109: F, t817: F, t1161: F, t2376: F, t830: F, t829: F) -> (F, F, F) {
    let t3015 = t3013 * t159 * t285;
    let t3030 = t1109 * t817;
    let t3045 = t2376 * t1161;
    let t3046 = t830 * t3045;
    let t3047 = t829 * t3046;
    (t3015, t3030, t3047)
}
