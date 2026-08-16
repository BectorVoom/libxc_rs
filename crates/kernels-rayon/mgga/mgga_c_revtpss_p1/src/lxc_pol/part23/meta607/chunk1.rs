//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2270/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2270(t12367: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t448: f64) -> (f64, f64) {
    let t24252 = -t12367 + 0.12361111111111111111e-1_f64 * t16706 + 0.61805555555555555556e-2_f64 * t20283 - 0.18541666666666666667e-1_f64 * t20285 - 0.92708333333333333334e-2_f64 * t20287 + 0.10300925925925925926e-1_f64 * t24230 - 0.37083333333333333333e-1_f64 * t24234 - 0.18541666666666666666e-1_f64 * t24238 + 0.55625000000000000001e-1_f64 * t24242 + 0.55625000000000000001e-1_f64 * t24246 + 0.92708333333333333333e-2_f64 * t24250;
    let t24253 = t24252 * t448;
    (t24252, t24253)
}
