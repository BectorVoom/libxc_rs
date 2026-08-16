//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1342/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1342(t114484: f64, t114513: f64, t114611: f64, t114632: f64, t114664: f64, t114701: f64, t114718: f64, t114740: f64, t1450: f64, t2014: f64, t532: f64, t196: f64, t197: f64, t22758: f64) -> (f64, f64) {
    let t114746 = t2014 * t532 * (t114484 + t114513 + t114611 + t114632 + t114664 + t114701 + t114718 + t114740) * t1450;
    let t114752 = t22758 * t196 * t197;
    (t114746, t114752)
}
