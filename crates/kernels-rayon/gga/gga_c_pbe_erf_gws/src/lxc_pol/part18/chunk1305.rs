//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1305/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1305(t56669: f64, t829: f64, t830: f64, t14767: f64, t3047: f64, t28652: f64, t3808: f64, t3972: f64, t3975: f64, t361: f64, t56296: f64, t13917: f64, t3223: f64) -> (f64, f64, f64, f64) {
    let t56671 = t829 * t830 * t56669;
    let t56674 = t14767 * t3047;
    let t56678 = t3972 * t3975 * t3808 * t28652;
    let t56684 = t361 * t56296;
    let t56686 = t13917 * t56684 * t3223;
    (t56671, t56674, t56678, t56686)
}
