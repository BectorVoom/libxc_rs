//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 890/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk890(t338: f64, t3722: f64, t892: f64, t2409: f64, t3212: f64, t8589: f64, t3060: f64, t8713: f64, t9283: f64, t3724: f64, t840: f64, t1161: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9865 = t338 * t892 * t3722;
    let t9869 = t2409 * t8589 * t3212;
    let t9872 = t8713 * t3060;
    let t9873 = t9283 * t9872;
    let t9879 = t840 * t3724;
    let t9883 = t8589 * t1161;
    (t9865, t9869, t9872, t9873, t9879, t9883)
}
