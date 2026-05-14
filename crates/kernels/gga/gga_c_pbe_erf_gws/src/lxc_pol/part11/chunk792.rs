//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 792/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk792<F: Float>(t13534: F, t850: F, t852: F, t860: F, t1076: F, t1109: F, t2255: F, t3258: F, t13252: F, t9607: F, t1153: F, t13523: F, t2118: F, t9499: F, t13187: F, t2300: F, t904: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13536 = t850 * t13534 * t852;
    let t13538 = t13536 * t860 / 96.0;
    let t13539 = t1076 * t1109;
    let t13541 = t2255 * t3258 * t13539;
    let t13544 = t9607 * t13252;
    let t13545 = t1153 * t13544;
    let t13548 = t2118 * t13523;
    let t13549 = t9499 * t13548;
    let t13553 = t2300 * t904 * t13187;
    (t13536, t13538, t13539, t13541, t13544, t13545, t13548, t13549, t13553)
}
