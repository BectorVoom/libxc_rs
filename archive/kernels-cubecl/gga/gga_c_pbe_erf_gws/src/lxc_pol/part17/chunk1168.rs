//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1168/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1168<F: Float>(t2387: F, t6792: F, t2365: F, t56: F, t2118: F, t822: F, t2306: F, t4383: F, t2382: F, t824: F, t376: F, t6161: F) -> (F, F, F, F, F, F, F) {
    let t19704 = t2387 * t6792;
    let t19775 = t2365 * t56;
    let t19776 = t2118 * t19775;
    let t19777 = t822 * t19776;
    let t19894 = t2306 * t4383;
    let t19895 = t2382 * t19894;
    let t19905 = t824 * t19775;
    let t19906 = t822 * t19905;
    let t19911 = t376 * t6161;
    (t19704, t19776, t19777, t19895, t19905, t19906, t19911)
}
