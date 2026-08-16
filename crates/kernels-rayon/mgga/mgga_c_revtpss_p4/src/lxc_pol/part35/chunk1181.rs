//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1181/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1181(t23257: f64, t25262: f64, t23285: f64, t7038: f64, t23342: f64, t7045: f64, t23289: f64, t23253: f64, t93062: f64, t1579: f64, t231: f64, t6016: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113228 = t25262 * t23257;
    let t113230 = t7038 * t23285;
    let t113232 = t7045 * t23342;
    let t113235 = t7038 * t23289;
    let t113237 = t93062 * t23253;
    let t113269 = t1579 * t6016 * t231;
    (t113228, t113230, t113232, t113235, t113237, t113269)
}
