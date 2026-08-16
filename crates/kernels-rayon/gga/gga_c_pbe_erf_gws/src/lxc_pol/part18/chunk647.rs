//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 647/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk647(t3553: f64, t650: f64, t186: f64, t211: f64, t1033: f64, t1046: f64, t1024: f64, t2741: f64, t3345: f64, t220: f64, t616: f64, t3515: f64, t3517: f64, t3521: f64, t3525: f64, t3529: f64, t3533: f64, t3537: f64, t3538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3554 = t650 * t3553;
    let t3555 = t186 * t3554;
    let t3557 = 2.0_f64 / 15.0_f64 * t211 * t3555;
    let t3559 = 4.0_f64 / 15.0_f64 * t1033 * t1046;
    let t3561 = 8.0_f64 / 15.0_f64 * t2741 * t1024;
    let t3562 = -t3345;
    let t3563 = t220 * t3562;
    let t3564 = t186 * t3563;
    let t3566 = 4.0_f64 / 15.0_f64 * t616 * t3564;
    let t3567 = -t3515 + t3517 + t3521 + t3525 + t3529 + t3533 - t3537 + t3538 - t3557 - t3559 + t3561 + t3566;
    (t3554, t3555, t3557, t3559, t3561, t3562, t3563, t3564, t3566, t3567)
}
