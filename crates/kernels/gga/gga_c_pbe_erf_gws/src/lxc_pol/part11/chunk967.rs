//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 967/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk967<F: Float>(t1114: F, t21764: F, t19810: F, t1120: F, t21681: F, t1164: F, t6729: F, t1150: F, t21117: F, t1112: F, t19561: F, t1154: F, t20646: F) -> (F, F, F, F, F, F, F) {
    let t28394 = t1114 * t21764;
    let t28397 = t1114 * t19810;
    let t28413 = t21681 * t1120;
    let t28487 = t6729 * t1164;
    let t28923 = t21117 * t1150;
    let t28975 = t1112 * t19561;
    let t29599 = t20646 * t1154;
    (t28394, t28397, t28413, t28487, t28923, t28975, t29599)
}
