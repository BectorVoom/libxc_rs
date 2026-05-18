//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 863/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk863<F: Float>(t13252: F, t9607: F, t1153: F, t13523: F, t2118: F, t9499: F, t13187: F, t2300: F, t904: F, t13461: F, t916: F, t11464: F, t2345: F, t3814: F) -> (F, F, F, F, F, F, F) {
    let t13544 = t9607 * t13252;
    let t13545 = t1153 * t13544;
    let t13548 = t2118 * t13523;
    let t13549 = t9499 * t13548;
    let t13553 = t2300 * t904 * t13187;
    let t13557 = t916 * t904 * t13461;
    let t13561 = t2345 * t11464 * t3814;
    (t13544, t13545, t13548, t13549, t13553, t13557, t13561)
}
