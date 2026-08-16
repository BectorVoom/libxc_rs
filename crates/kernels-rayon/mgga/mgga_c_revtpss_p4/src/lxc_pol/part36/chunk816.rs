//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 816/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk816(t225: f64, t9801: f64, t4062: f64, t1386: f64, t2482: f64, t814: f64, t136: f64, t1412: f64, t220: f64, t2735: f64, t4086: f64, t521: f64, t9342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9802 = t9801 * t225;
    let t9804 = 0.45738002528356795401e-4_f64 * t9802 * t4062;
    let t9816 = t2482 * t1386 * t814;
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9845 = t2735 * t4086;
    let t9854 = 24.0_f64 * t9342 * t521;
    (t9802, t9804, t9816, t9818, t9845, t9854)
}
