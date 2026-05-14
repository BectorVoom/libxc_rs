//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 872/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk872<F: Float>(t1154: F, t20646: F, t1158: F, t21507: F, t1326: F, t3360: F, t40: F, t11262: F, t1371: F, t553: F, t3379: F, t784: F, t1378: F, t1971: F, t3562: F, t5212: F) -> (F, F, F, F, F, F, F) {
    let t29599 = t20646 * t1154;
    let t29638 = t21507 * t1158;
    let t30116 = t40 * t3360 * t1326;
    let t30127 = t11262 * t1371 * t553;
    let t30129 = t784 * t3379;
    let t30131 = t30129 * t1378 * t1971;
    let t30170 = t5212 * t3562;
    (t29599, t29638, t30116, t30127, t30129, t30131, t30170)
}
