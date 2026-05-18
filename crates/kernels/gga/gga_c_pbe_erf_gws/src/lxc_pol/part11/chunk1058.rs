//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1058/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1058<F: Float>(t13249: F, t6402: F, t3123: F, t37138: F, t12041: F, t36666: F, t13342: F, t6416: F, t13124: F, t19561: F, t13446: F, t2206: F) -> (F, F, F, F, F, F) {
    let t45755 = t6402 * t13249;
    let t45767 = t3123 * t37138;
    let t45771 = t12041 * t36666;
    let t45793 = t6416 * t13342;
    let t45805 = t13124 * t19561;
    let t45821 = t2206 * t13446;
    (t45755, t45767, t45771, t45793, t45805, t45821)
}
