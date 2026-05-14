//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 328/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk328<F: Float>(t40: F, t960: F, t85: F, t959: F, t476: F, t950: F, t478: F, t954: F) -> (F, F, F, F, F) {
    let t961 = t40 * t960;
    let t962 = t959 * t85;
    let t963 = 0.19751789702565206229e-1 * t962;
    let t964 = t476 * t950;
    let t965 = t478 * t954;
    let t967 = t964 / 3.0 + t965 / 3.0;
    (t961, t963, t964, t965, t967)
}
