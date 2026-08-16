//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 821/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk821(t2042: f64, t2170: f64, t573: f64, t8609: f64, t8613: f64, t8616: f64, t8771: f64, t3140: f64, t3736: f64, t1276: f64, t1243: f64, t197: f64, t532: f64) -> (f64, f64, f64, f64) {
    let t8773 = t2170 * t2042;
    let t8776 = t573 * t8771 + 3.0_f64 * t8609 + t8613 + t8616 + 3.0_f64 * t8773;
    let t8939 = t3140 * t3736;
    let t8944 = t3140 * t1276;
    let t8945 = t8944 * t1243;
    let t8995 = t197 * t532;
    (t8776, t8939, t8945, t8995)
}
