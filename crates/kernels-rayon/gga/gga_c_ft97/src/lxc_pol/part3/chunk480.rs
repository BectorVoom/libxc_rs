//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 480/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk480(t147: f64, t184: f64, t3658: f64, t21: f64, t1078: f64, t648: f64, t1079: f64, t363: f64, t649: f64, t920: f64, t18: f64, t1577: f64, t1064: f64, t1080: f64, t2240: f64, t3597: f64, t3601: f64, t5: f64, t620: f64, t623: f64, t650: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t3659 = t3658 * t184;
    let t3660 = t3659 * t21;
    let t3663 = t1078 * t648;
    let t3664 = t184 * t21;
    let t3665 = t3663 * t3664;
    let t3668 = t1079 * t363;
    let t3674 = t649 * t920;
    let t3677 = t184 * t18;
    let t3678 = t3677 * t1577;
    let t3682 = piecewise3(t148, 0.0_f64, t5 * t3597 * t21 / 4.0_f64 + t3601 * t650 / 4.0_f64 + t5 * t1064 * t363 / 4.0_f64 + t2240 * t1080 / 4.0_f64 + t623 * t3660 / 4.0_f64 + t623 * t3665 / 4.0_f64 + t623 * t3668 / 4.0_f64 + t5 * t620 * t920 / 4.0_f64 + t623 * t3674 / 4.0_f64 + t623 * t3678 / 2.0_f64);
    (t3659, t3660, t3663, t3664, t3665, t3668, t3674, t3678, t3682)
}
