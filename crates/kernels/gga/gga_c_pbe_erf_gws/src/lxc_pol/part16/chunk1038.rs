//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1038/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1038<F: Float>(t19775: F, t824: F, t822: F, t376: F, t6161: F, t2169: F, t2200: F, t329: F, t2271: F, t4383: F, t2409: F) -> (F, F, F, F, F, F, F) {
    let t19905 = t824 * t19775;
    let t19906 = t822 * t19905;
    let t19911 = t376 * t6161;
    let t20091 = t329 * t2200 * t2169;
    let t20112 = t2271 * t4383;
    let t20113 = t822 * t20112;
    let t20154 = t2169 * t2409;
    (t19905, t19906, t19911, t20091, t20112, t20113, t20154)
}
