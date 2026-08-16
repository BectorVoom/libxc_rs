//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 906/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk906(t2559: f64, t7900: f64, t1820: f64, t2756: f64, t579: f64, t532: f64, t4358: f64, t4561: f64) -> (f64, f64, f64) {
    let t7901 = t2559 * t7900;
    let t7903 = 8.0_f64 / 27.0_f64 * t1820 * t7901;
    let t7905 = 8.0_f64 / 45.0_f64 * t579 * t2756;
    let t7906 = 4.0_f64 * t532;
    let t7907 = 12.0_f64 * t4358;
    let t7908 = -t7906 - t7907 + t4561;
    (t7903, t7905, t7908)
}
