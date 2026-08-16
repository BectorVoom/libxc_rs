//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3399/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3399(t41441: f64, t63462: f64, t63464: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t63554: f64, t63557: f64, t63560: f64, t63563: f64, t63566: f64, t63568: f64) -> f64 {
    let t63813 = -0.22076e0_f64 * t63541 + 0.36793333333333333334e-1_f64 * t63543 - 0.91983333333333333334e-1_f64 * t63545 - 0.22076e0_f64 * t63547 + 0.73586666666666666667e-1_f64 * t63549 + 0.49057777777777777778e-1_f64 * t63551 + 0.16557e0_f64 * t63554 + 0.44152e0_f64 * t63557 - 0.5519e-1_f64 * t63560 - 0.36793333333333333333e-1_f64 * t63563 - 0.8585111111111111111e-1_f64 * t63566 - 0.258925e1_f64 * t63568 + 0.49057777777777777778e0_f64 * t41441 + 0.12077e1_f64 * t63462 - 0.13418888888888888889e0_f64 * t63464;
    t63813
}
