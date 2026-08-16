//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 977/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk977(t1162: f64, t810: f64, t353: f64, t4386: f64, t1118: f64, t814: f64, t3037: f64, t328: f64, t2306: f64, t3074: f64, t2501: f64, t2370: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8693 = t1162 * t810;
    let t8694 = t353 * t8693;
    let t8695 = t4386 * t8694;
    let t8698 = t1118 * t814;
    let t8699 = t353 * t8698;
    let t8700 = t4386 * t8699;
    let t8703 = t3037 * t328;
    let t8704 = t2306 * t8703;
    let t8705 = t3074 * t8704;
    let t8708 = t2501 * t810;
    let t8710 = t2370 * t830 * t8708;
    (t8695, t8700, t8703, t8705, t8708, t8710)
}
