//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1995/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1995(t7064: f64, t93150: f64, t7015: f64, t9292: f64, t25411: f64, t93183: f64, t25387: f64, t93285: f64, t7063: f64, t860: f64, t25374: f64, t11007: f64, t1955: f64, t7056: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93324 = t7064 * t93150;
    let t93334 = 0.17073386770573548589e-1_f64 * t9292 * t7015;
    let t93335 = t25411 * t93183;
    let t93339 = t25387 * t93285;
    let t93341 = t7063 * t860;
    let t93342 = t93341 * t25374;
    let t93349 = t1955 * t7056 * t11007;
    (t93324, t93334, t93335, t93339, t93341, t93342, t93349)
}
