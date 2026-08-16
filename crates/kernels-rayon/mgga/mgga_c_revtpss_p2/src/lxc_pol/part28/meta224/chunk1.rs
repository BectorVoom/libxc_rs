//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1057/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1057(t1469: f64, t3362: f64, t606: f64, t3360: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t5046 = t3362 * t1469;
    let t5047 = t5046 * t606;
    let t5048 = t3360 * t5047;
    let t5049 = t128 * t5048;
    (t5046, t5047, t5048, t5049)
}
