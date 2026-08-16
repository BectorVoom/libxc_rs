//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 814/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk814(t6330: f64, t6334: f64, t6338: f64, t6344: f64, t6375: f64, t6413: f64, t6415: f64, t6444: f64, t6446: f64, t6448: f64, t6461: f64, t6482: f64, t6486: f64, t6490: f64) -> f64 {
    let t6733 = t6330 + t6334 - t6338 + t6344 - t6375 - t6413 - t6415 - t6444 + t6446 + t6448 + t6461 - t6482 - t6486 + t6490;
    t6733
}
