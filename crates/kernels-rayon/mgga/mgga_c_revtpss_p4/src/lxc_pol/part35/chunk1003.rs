//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1003/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1003(t12610: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t482: f64) -> (f64, f64) {
    let t24633 = -t12610 + 0.19755555555555555556e-1_f64 * t16706 + 0.9877777777777777778e-2_f64 * t20283 - 0.29633333333333333334e-1_f64 * t20285 - 0.14816666666666666667e-1_f64 * t20287 + 0.16462962962962962963e-1_f64 * t24230 - 0.59266666666666666668e-1_f64 * t24234 - 0.29633333333333333334e-1_f64 * t24238 + 0.88900000000000000002e-1_f64 * t24242 + 0.88900000000000000002e-1_f64 * t24246 + 0.14816666666666666667e-1_f64 * t24250;
    let t24634 = t482 * t24633;
    (t24633, t24634)
}
