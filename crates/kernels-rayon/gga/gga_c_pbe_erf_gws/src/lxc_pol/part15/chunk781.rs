//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 781/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk781(t118: f64, t119: f64, t120: f64, t837: f64, t1552: f64, t19: f64, t506: f64, t299: f64, t481: f64, t799: f64, t1533: f64, t155: f64) -> (f64, f64, f64, f64) {
    let t5759 = 7.0_f64 / 27.0_f64 * t118 * t119 * t837 * t120;
    let t5761 = t1552 * t506 * t19;
    let t5763 = t799 * t299 * t481;
    let t5764 = t5761 * t5763;
    let t5767 = t119 * t155 * t1533;
    (t5759, t5763, t5764, t5767)
}
