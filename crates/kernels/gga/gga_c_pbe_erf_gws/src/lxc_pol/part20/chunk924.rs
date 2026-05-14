//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 924/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk924<F: Float>(t11422: F, t9499: F, t824: F, t8895: F, t9125: F, t3222: F, t9607: F, t1153: F, t8989: F, t1134: F, t820: F, t2306: F, t9386: F, t3123: F, t8824: F, t11416: F, t11418: F, t11421: F, t6275: F, t6637: F, t8823: F, t9342: F, t9637: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11423 = t9499 * t11422;
    let t11426 = t824 * t8895;
    let t11427 = t9499 * t11426;
    let t11430 = t824 * t9125;
    let t11431 = t9499 * t11430;
    let t11434 = t9607 * t3222;
    let t11435 = t1153 * t11434;
    let t11438 = t824 * t8989;
    let t11439 = t9499 * t11438;
    let t11442 = t1134 * t820;
    let t11443 = t2306 * t11442;
    let t11444 = t9386 * t11443;
    let t11447 = t3123 * t8824;
    let t11448 = 7.0 / 144.0 * t11447;
    let t11449 = t11416 + t11418 + t11421 + t6637 * t11423 / 384.0 + t6275 * t11427 / 96.0 + t6275 * t11431 / 96.0 + t6275 * t11435 / 96.0 + t9637 * t11439 / 128.0 - t6637 * t11444 / 192.0 + t11448 + t8823 + t9342;
    (t11423, t11426, t11427, t11430, t11431, t11434, t11435, t11438, t11439, t11443, t11444, t11448, t11449)
}
