//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 803/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk803<F: Float>(t16669: F, t4951: F, t11: F, t1758: F, t1403: F, t1407: F, t4957: F) -> (F, F, F, F, F) {
    let t16670 = t4951 * t16669;
    let t16672 = t11 * t1758 * t16670;
    let t16675 = t4957 * t1403 * t1407;
    let t16677 = t11 * t1758 * t16675;
    let t16679 = t1407 * t1407;
    (t16670, t16672, t16675, t16677, t16679)
}
