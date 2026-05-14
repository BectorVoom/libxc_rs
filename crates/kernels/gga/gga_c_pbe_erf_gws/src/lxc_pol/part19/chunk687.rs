//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 687/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk687<F: Float>(t3956: F, t3980: F, t1205: F, t2376: F, t830: F, t829: F) -> (F, F, F, F) {
    let t4072 = 7.0 / 144.0 * t3956;
    let t4077 = 7.0 / 2304.0 * t3980;
    let t4081 = t2376 * t1205;
    let t4082 = t830 * t4081;
    let t4083 = t829 * t4082;
    (t4072, t4077, t4082, t4083)
}
