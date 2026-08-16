//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1186/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1186(t3938: f64, t9818: f64, t9819: f64, t9816: f64, t4003: f64, t4056: f64, t2735: f64, t4086: f64, t3994: f64, t808: f64, t521: f64, t9342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9840 = t4003 * t4056;
    let t9845 = t2735 * t4086;
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9854 = 24.0_f64 * t9342 * t521;
    (t9821, t9822, t9840, t9845, t9847, t9854)
}
