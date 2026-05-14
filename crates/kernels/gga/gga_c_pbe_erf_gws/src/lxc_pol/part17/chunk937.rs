//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 937/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk937<F: Float>(t2255: F, t9433: F, t2279: F, t9364: F, t1112: F, t4394: F, t6665: F, t3253: F, t6203: F, t904: F, t9080: F, t916: F, t2300: F, t8759: F, t2253: F, t2277: F, t8925: F, t8927: F, t8930: F, t8932: F, t8936: F, t8938: F, t914: F, t929: F) -> (F, F, F, F, F, F, F) {
    let t9434 = t2255 * t9433;
    let t9438 = t2255 * t9364 * t2279;
    let t9441 = t1112 * t4394;
    let t9443 = t2255 * t9441 * t6665;
    let t9447 = 7.0 / 288.0 * t6203 * t3253;
    let t9449 = t916 * t904 * t9080;
    let t9453 = t2300 * t904 * t8759;
    let t9456 = -t8925 - t2253 * t9434 / 384.0 - t8927 - t8930 + t8932 - t2277 * t9438 / 768.0 + t2277 * t9443 / 768.0 + t8936 - t8938 + t9447 - t914 * t9449 / 1536.0 + 5.0 / 768.0 * t929 * t9453;
    (t9434, t9438, t9441, t9443, t9449, t9453, t9456)
}
