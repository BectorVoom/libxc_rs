//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 827/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk827(t2115: f64, t2170: f64, t573: f64, t8616: f64, t8728: f64, t8733: f64, t8905: f64, t3140: f64, t3736: f64, t1276: f64, t1243: f64, t197: f64, t532: f64) -> (f64, f64, f64, f64) {
    let t8909 = 3.0_f64 * t2115 * t2170 + t573 * t8905 + t8616 + t8728 + t8733;
    let t8939 = t3140 * t3736;
    let t8944 = t3140 * t1276;
    let t8945 = t8944 * t1243;
    let t8995 = t197 * t532;
    (t8909, t8939, t8945, t8995)
}
