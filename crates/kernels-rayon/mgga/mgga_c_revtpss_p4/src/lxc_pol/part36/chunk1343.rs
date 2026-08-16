//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1343/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1343(t114752: f64, t2035: f64, t29499: f64, t7898: f64, t29495: f64, t29506: f64, t7937: f64, t2014: f64, t2034: f64, t86791: f64, t30112: f64, t7935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114753 = t114752 * t2035;
    let t114755 = 18.0_f64 * t7898 * t29499;
    let t114757 = 9.0_f64 * t7898 * t29495;
    let t114759 = 3.0_f64 * t29506 * t7937;
    let t114765 = 6.0_f64 * t2014 * t2034 * t86791;
    let t114768 = 3.0_f64 * t7898 * t30112;
    let t114770 = 3.0_f64 * t29506 * t7935;
    (t114753, t114755, t114757, t114759, t114765, t114768, t114770)
}
