//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 237/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk237<F: Float>(t168: F, t270: F, t703: F, t247: F, t535: F, t251: F, t147: F, t19: F, t336: F) -> (F, F, F, F, F) {
    let t706 = 0.19897291109174608293e-1 * t168 * t703 * t270;
    let t707 = t535 * t247;
    let t708 = t707 * t251;
    let t711 = t147 * t19;
    let t712 = t711 * t336;
    (t706, t707, t708, t711, t712)
}
