//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1056/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1056<F: Float>(t876: F, t9246: F, t2134: F, t14046: F, t14096: F, t2216: F, t4033: F, t14058: F, t2327: F, t14079: F, t2285: F, t1185: F, t326: F, t346: F, t6045: F, t2212: F) -> (F, F, F, F, F, F, F, F) {
    let t51430 = t9246 * t876;
    let t51431 = t2134 * t51430;
    let t51437 = t14046 * t14096;
    let t51439 = t4033 * t2216;
    let t51447 = t14058 * t2327;
    let t51452 = t14079 * t2285;
    let t51458 = t326 * t346 * t6045 * t1185;
    let t51461 = t4033 * t2212;
    (t51430, t51431, t51437, t51439, t51447, t51452, t51458, t51461)
}
