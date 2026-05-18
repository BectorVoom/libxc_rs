//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1084/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1084<F: Float>(t13872: F, t3965: F, t4018: F, t9270: F, t2307: F, t3975: F, t3972: F, t1193: F, t2220: F, t338: F, t4055: F, t840: F) -> (F, F, F, F, F, F) {
    let t13873 = t3965 * t13872;
    let t13875 = t9270 * t4018;
    let t13877 = t3975 * t2307;
    let t13878 = t3972 * t13877;
    let t13881 = t338 * t2220 * t1193;
    let t13884 = t840 * t4055;
    (t13873, t13875, t13877, t13878, t13881, t13884)
}
