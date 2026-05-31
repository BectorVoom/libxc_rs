//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1022/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1022<F: Float>(t11438: F, t9499: F, t1134: F, t820: F, t2306: F, t9386: F, t3123: F, t8824: F, t11416: F, t11418: F, t11421: F, t11423: F, t11427: F, t11431: F, t11435: F, t6275: F, t6637: F, t8823: F, t9342: F, t9637: F) -> (F, F, F, F, F) {
    let t11439 = t9499 * t11438;
    let t11442 = t1134 * t820;
    let t11443 = t2306 * t11442;
    let t11444 = t9386 * t11443;
    let t11447 = t3123 * t8824;
    let t11448 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t11447;
    let t11449 = t11416 + t11418 + t11421 + t6637 * t11423 / F::cast_from(384.0_f64) + t6275 * t11427 / F::cast_from(96.0_f64) + t6275 * t11431 / F::cast_from(96.0_f64) + t6275 * t11435 / F::cast_from(96.0_f64) + t9637 * t11439 / F::cast_from(128.0_f64) - t6637 * t11444 / F::cast_from(192.0_f64) + t11448 + t8823 + t9342;
    (t11439, t11443, t11444, t11448, t11449)
}
