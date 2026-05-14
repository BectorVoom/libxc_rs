//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 972/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk972<F: Float>(t13972: F, t3993: F, t2409: F, t4400: F, t3965: F, t859: F, t940: F) -> (F, F, F, F, F) {
    let t13973 = t13972 * t3993;
    let t13974 = 7.0 / 2304.0 * t13973;
    let t13976 = t2409 * t4400;
    let t13977 = t3965 * t13976;
    let t13984 = t859 * t940;
    (t13973, t13974, t13976, t13977, t13984)
}
