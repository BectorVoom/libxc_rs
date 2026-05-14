//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 727/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk727<F: Float>(t256: F, t5427: F, t1910: F, t723: F, t1924: F, t1917: F, t245: F, t712: F, t1903: F, t708: F, t703: F, t713: F, t1906: F, t719: F, t1354: F, t19: F) -> (F, F, F, F, F, F, F, F) {
    let t5429 = t5427 * t256 / 3.0;
    let t5430 = t1910 * t723;
    let t5433 = 2.0 / 3.0 * t1924 * t723;
    let t5434 = t245 * t1917;
    let t5436 = 0.2e-20 * t712 * t5434;
    let t5437 = t708 * t1903;
    let t5441 = t703 * t713;
    let t5443 = 0.13506172839506172839e-1 * t712 * t5441;
    let t5448 = t1906 * t719;
    let t5449 = t5448 * t256;
    let t5450 = t1354 * t19;
    (t5429, t5430, t5433, t5436, t5437, t5443, t5449, t5450)
}
