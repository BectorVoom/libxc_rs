//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 343/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk343(t1017: f64, t598: f64, t186: f64, t185: f64, t995: f64) -> (f64, f64, f64, f64) {
    let t1018 = t598 * t1017;
    let t1019 = t186 * t1018;
    let t1021 = 2.0_f64 / 15.0_f64 * t185 * t1019;
    let t1022 = -t995;
    (t1018, t1019, t1021, t1022)
}
