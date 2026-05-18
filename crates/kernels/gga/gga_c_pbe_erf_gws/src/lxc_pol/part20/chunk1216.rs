//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1216/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1216<F: Float>(t13806: F, t2276: F, t932: F, t2118: F, t2132: F, t2263: F, t331: F, t56: F, t863: F, t14022: F, t885: F, t2149: F) -> (F, F, F, F, F) {
    let t51255 = t2276 * t13806 * t932;
    let t51266 = t2118 * t2132;
    let t51274 = t863 * t2263 * t331 * t56;
    let t51291 = t14022 * t885;
    let t51292 = t51291 * t2149;
    (t51255, t51266, t51274, t51291, t51292)
}
