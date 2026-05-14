//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1062/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1062<F: Float>(t19775: F, t2118: F, t2306: F, t4383: F, t4395: F, t824: F, t822: F, t2169: F, t2200: F, t329: F, t2271: F, t2409: F) -> (F, F, F, F, F, F, F, F) {
    let t19776 = t2118 * t19775;
    let t19894 = t2306 * t4383;
    let t19898 = t4395 * t4383;
    let t19905 = t824 * t19775;
    let t19906 = t822 * t19905;
    let t20091 = t329 * t2200 * t2169;
    let t20112 = t2271 * t4383;
    let t20154 = t2169 * t2409;
    (t19776, t19894, t19898, t19905, t19906, t20091, t20112, t20154)
}
