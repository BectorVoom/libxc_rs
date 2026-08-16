//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 942/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk942(t10201: f64, t247: f64, t251: f64, t3583: f64, t719: f64, t256: f64, t19: f64, t3379: f64, t336: f64, t714: f64, t1046: f64, t2816: f64) -> (f64, f64, f64, f64) {
    let t10602 = t10201 * t247;
    let t10603 = t10602 * t251;
    let t10606 = t3583 * t719;
    let t10607 = t10606 * t256;
    let t10609 = t3379 * t19;
    let t10610 = t10609 * t336;
    let t10611 = t10610 * t714;
    let t10614 = 4.0_f64 / 15.0_f64 * t2816 * t1046;
    (t10603, t10607, t10611, t10614)
}
